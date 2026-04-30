#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, symbol_short};

#[derive(Clone)]
#[contracttype]
pub struct Subscription {
    pub user: Address,
    pub service: Symbol,
    pub amount: i128,
    pub next_payment: u64,
    pub interval: u64, // saniye cinsinden (örn: 30 gün = 2592000)
    pub active: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Subscription(Address, Symbol), // (user, service)
    ServiceProvider(Symbol),       // service -> provider address
}

#[contract]
pub struct StellarSubsContract;

#[contractimpl]
impl StellarSubsContract {
    /// Yeni bir servis sağlayıcı kaydeder
    pub fn register_service(
        env: Env,
        provider: Address,
        service: Symbol,
    ) {
        provider.require_auth();
        
        let key = DataKey::ServiceProvider(service.clone());
        env.storage().persistent().set(&key, &provider);
    }

    /// Yeni abonelik oluşturur
    pub fn create_subscription(
        env: Env,
        user: Address,
        service: Symbol,
        amount: i128,
        interval: u64,
    ) {
        user.require_auth();
        
        // Servis sağlayıcının kayıtlı olduğunu kontrol et
        let provider_key = DataKey::ServiceProvider(service.clone());
        let _provider: Address = env.storage().persistent()
            .get(&provider_key)
            .expect("Service not registered");
        
        let next_payment = env.ledger().timestamp() + interval;
        
        let subscription = Subscription {
            user: user.clone(),
            service: service.clone(),
            amount,
            next_payment,
            interval,
            active: true,
        };
        
        let key = DataKey::Subscription(user, service);
        env.storage().persistent().set(&key, &subscription);
    }

    /// Abonelik ödemesi yapar
    pub fn process_payment(
        env: Env,
        user: Address,
        service: Symbol,
    ) {
        user.require_auth();
        
        let key = DataKey::Subscription(user.clone(), service.clone());
        let mut subscription: Subscription = env.storage().persistent()
            .get(&key)
            .expect("Subscription not found");
        
        // Aboneliğin aktif olduğunu kontrol et
        assert!(subscription.active, "Subscription is not active");
        
        // Ödeme zamanının geldiğini kontrol et
        let current_time = env.ledger().timestamp();
        assert!(current_time >= subscription.next_payment, "Payment not due yet");
        
        // Servis sağlayıcıyı al
        let provider_key = DataKey::ServiceProvider(service.clone());
        let provider: Address = env.storage().persistent()
            .get(&provider_key)
            .expect("Service provider not found");
        
        // Token transferi (native XLM için)
        // Not: Gerçek implementasyonda token contract çağrısı yapılmalı
        env.events().publish(
            (symbol_short!("payment"), user.clone(), service.clone()),
            (provider, subscription.amount)
        );
        
        // Bir sonraki ödeme tarihini güncelle
        subscription.next_payment = current_time + subscription.interval;
        env.storage().persistent().set(&key, &subscription);
    }

    /// Aboneliği iptal eder
    pub fn cancel_subscription(
        env: Env,
        user: Address,
        service: Symbol,
    ) {
        user.require_auth();
        
        let key = DataKey::Subscription(user.clone(), service.clone());
        let mut subscription: Subscription = env.storage().persistent()
            .get(&key)
            .expect("Subscription not found");
        
        subscription.active = false;
        env.storage().persistent().set(&key, &subscription);
    }

    /// Abonelik bilgilerini getirir
    pub fn get_subscription(
        env: Env,
        user: Address,
        service: Symbol,
    ) -> Option<Subscription> {
        let key = DataKey::Subscription(user, service);
        env.storage().persistent().get(&key)
    }

    /// Servis sağlayıcı adresini getirir
    pub fn get_service_provider(
        env: Env,
        service: Symbol,
    ) -> Option<Address> {
        let key = DataKey::ServiceProvider(service);
        env.storage().persistent().get(&key)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, Symbol};

    #[test]
    fn test_subscription_flow() {
        let env = Env::default();
        let contract_id = env.register_contract(None, StellarSubsContract);
        let client = StellarSubsContractClient::new(&env, &contract_id);

        let provider = Address::generate(&env);
        let user = Address::generate(&env);
        let service = Symbol::new(&env, "spotify");

        // Mock auth
        env.mock_all_auths();

        // Servis kaydı
        client.register_service(&provider, &service);

        // Abonelik oluştur
        let amount = 1000_0000000; // 10 XLM
        let interval = 2592000; // 30 gün
        client.create_subscription(&user, &service, &amount, &interval);

        // Abonelik bilgilerini kontrol et
        let subscription = client.get_subscription(&user, &service);
        assert!(subscription.is_some());
        assert_eq!(subscription.unwrap().active, true);

        // Aboneliği iptal et
        client.cancel_subscription(&user, &service);
        let subscription = client.get_subscription(&user, &service);
        assert_eq!(subscription.unwrap().active, false);
    }
}
