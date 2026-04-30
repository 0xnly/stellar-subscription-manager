# StellarSubs Kullanım Örnekleri

## 🎯 Gerçek Dünya Senaryoları

### Senaryo 1: Spotify Aboneliği

```bash
# 1. Spotify provider olarak kayıt ol
soroban keys generate spotify --network testnet
soroban contract invoke \
  --id CONTRACT_ID \
  --source spotify \
  --network testnet \
  -- \
  register_service \
  --provider $(soroban keys address spotify) \
  --service spotify

# 2. Kullanıcı abonelik oluştur (10 XLM/ay)
soroban keys generate user_ahmet --network testnet
soroban contract invoke \
  --id CONTRACT_ID \
  --source user_ahmet \
  --network testnet \
  -- \
  create_subscription \
  --user $(soroban keys address user_ahmet) \
  --service spotify \
  --amount 100000000 \
  --interval 2592000

# 3. Aylık ödeme yap
soroban contract invoke \
  --id CONTRACT_ID \
  --source user_ahmet \
  --network testnet \
  -- \
  process_payment \
  --user $(soroban keys address user_ahmet) \
  --service spotify

# 4. Abonelik durumunu kontrol et
soroban contract invoke \
  --id CONTRACT_ID \
  --network testnet \
  -- \
  get_subscription \
  --user $(soroban keys address user_ahmet) \
  --service spotify
```

### Senaryo 2: Netflix Aboneliği

```bash
# Netflix provider kaydı
soroban keys generate netflix --network testnet
soroban contract invoke \
  --id CONTRACT_ID \
  --source netflix \
  --network testnet \
  -- \
  register_service \
  --provider $(soroban keys address netflix) \
  --service netflix

# Kullanıcı Premium plan (15 XLM/ay)
soroban contract invoke \
  --id CONTRACT_ID \
  --source user_ahmet \
  --network testnet \
  -- \
  create_subscription \
  --user $(soroban keys address user_ahmet) \
  --service netflix \
  --amount 150000000 \
  --interval 2592000
```

### Senaryo 3: LinkedIn Premium (Yıllık)

```bash
# LinkedIn provider kaydı
soroban keys generate linkedin --network testnet
soroban contract invoke \
  --id CONTRACT_ID \
  --source linkedin \
  --network testnet \
  -- \
  register_service \
  --provider $(soroban keys address linkedin) \
  --service linkedin

# Yıllık abonelik (100 XLM/yıl = 31536000 saniye)
soroban contract invoke \
  --id CONTRACT_ID \
  --source user_ahmet \
  --network testnet \
  -- \
  create_subscription \
  --user $(soroban keys address user_ahmet) \
  --service linkedin \
  --amount 1000000000 \
  --interval 31536000
```

### Senaryo 4: Abonelik İptali

```bash
# Spotify aboneliğini iptal et
soroban contract invoke \
  --id CONTRACT_ID \
  --source user_ahmet \
  --network testnet \
  -- \
  cancel_subscription \
  --user $(soroban keys address user_ahmet) \
  --service spotify

# İptal sonrası durumu kontrol et
soroban contract invoke \
  --id CONTRACT_ID \
  --network testnet \
  -- \
  get_subscription \
  --user $(soroban keys address user_ahmet) \
  --service spotify
# Output: active: false
```

## 💰 Fiyatlandırma Örnekleri

### Stroops Hesaplama
1 XLM = 10,000,000 stroops

| Servis | Aylık Fiyat (XLM) | Stroops | Interval (saniye) |
|--------|-------------------|---------|-------------------|
| Spotify | 10 XLM | 100000000 | 2592000 (30 gün) |
| Netflix Basic | 8 XLM | 80000000 | 2592000 |
| Netflix Premium | 15 XLM | 150000000 | 2592000 |
| YouTube Premium | 12 XLM | 120000000 | 2592000 |
| LinkedIn Premium | 30 XLM | 300000000 | 2592000 |
| GitHub Pro | 4 XLM | 40000000 | 2592000 |

### Interval Hesaplama

```javascript
// JavaScript örneği
const MINUTE = 60;
const HOUR = 60 * MINUTE;
const DAY = 24 * HOUR;
const WEEK = 7 * DAY;
const MONTH = 30 * DAY;
const YEAR = 365 * DAY;

// Haftalık abonelik
const weeklyInterval = WEEK; // 604800

// Aylık abonelik
const monthlyInterval = MONTH; // 2592000

// Yıllık abonelik
const yearlyInterval = YEAR; // 31536000
```

## 🔄 Otomatik Ödeme Sistemi

### Cron Job ile Otomatik Ödeme (Örnek)

```bash
#!/bin/bash
# auto-payment.sh

CONTRACT_ID="YOUR_CONTRACT_ID"
USER_KEY="user_ahmet"
SERVICES=("spotify" "netflix" "linkedin")

for service in "${SERVICES[@]}"; do
  echo "Processing payment for $service..."
  
  soroban contract invoke \
    --id $CONTRACT_ID \
    --source $USER_KEY \
    --network testnet \
    -- \
    process_payment \
    --user $(soroban keys address $USER_KEY) \
    --service $service
    
  if [ $? -eq 0 ]; then
    echo "✅ Payment successful for $service"
  else
    echo "❌ Payment failed for $service"
  fi
done
```

Crontab ekle (her gün kontrol et):
```bash
crontab -e
# Her gün saat 00:00'da çalıştır
0 0 * * * /path/to/auto-payment.sh
```

## 📊 Monitoring ve Analytics

### Tüm Abonelikleri Listele (Script)

```bash
#!/bin/bash
# list-subscriptions.sh

CONTRACT_ID="YOUR_CONTRACT_ID"
USER_ADDRESS=$(soroban keys address user_ahmet)
SERVICES=("spotify" "netflix" "youtube" "linkedin" "github")

echo "=== User Subscriptions ==="
echo "User: $USER_ADDRESS"
echo ""

for service in "${SERVICES[@]}"; do
  echo "Checking $service..."
  result=$(soroban contract invoke \
    --id $CONTRACT_ID \
    --network testnet \
    -- \
    get_subscription \
    --user $USER_ADDRESS \
    --service $service 2>&1)
  
  if [[ $result != *"not found"* ]]; then
    echo "✅ Active subscription: $service"
    echo "$result"
  else
    echo "❌ No subscription: $service"
  fi
  echo "---"
done
```

## 🌐 Frontend Entegrasyonu (JavaScript)

```javascript
import { SorobanRpc, Contract, Keypair } from '@stellar/stellar-sdk';

const CONTRACT_ID = 'YOUR_CONTRACT_ID';
const RPC_URL = 'https://soroban-testnet.stellar.org:443';

class StellarSubsClient {
  constructor(contractId, rpcUrl) {
    this.server = new SorobanRpc.Server(rpcUrl);
    this.contract = new Contract(contractId);
  }

  async createSubscription(userKeypair, service, amount, interval) {
    // Kontrat çağrısı oluştur
    const operation = this.contract.call(
      'create_subscription',
      userKeypair.publicKey(),
      service,
      amount,
      interval
    );

    // Transaction oluştur ve gönder
    // ... (detaylı implementation)
  }

  async getSubscription(userAddress, service) {
    const operation = this.contract.call(
      'get_subscription',
      userAddress,
      service
    );

    // ... (detaylı implementation)
    return subscription;
  }

  async processPayment(userKeypair, service) {
    const operation = this.contract.call(
      'process_payment',
      userKeypair.publicKey(),
      service
    );

    // ... (detaylı implementation)
  }
}

// Kullanım
const client = new StellarSubsClient(CONTRACT_ID, RPC_URL);

// Abonelik oluştur
await client.createSubscription(
  userKeypair,
  'spotify',
  100000000, // 10 XLM
  2592000    // 30 gün
);
```

## 🎨 UI Örnekleri

### React Component Örneği

```jsx
import { useState, useEffect } from 'react';

function SubscriptionCard({ service, amount, interval, active }) {
  const xlmAmount = amount / 10000000;
  const daysInterval = interval / 86400;

  return (
    <div className="subscription-card">
      <h3>{service}</h3>
      <p>Amount: {xlmAmount} XLM</p>
      <p>Billing: Every {daysInterval} days</p>
      <span className={active ? 'active' : 'inactive'}>
        {active ? '✅ Active' : '❌ Cancelled'}
      </span>
    </div>
  );
}

function SubscriptionList({ userAddress, contractId }) {
  const [subscriptions, setSubscriptions] = useState([]);

  useEffect(() => {
    // Fetch subscriptions from contract
    fetchSubscriptions();
  }, [userAddress]);

  return (
    <div className="subscription-list">
      {subscriptions.map(sub => (
        <SubscriptionCard key={sub.service} {...sub} />
      ))}
    </div>
  );
}
```

## 🔐 Güvenlik Best Practices

### 1. Private Key Yönetimi

```bash
# ASLA private key'leri commit etmeyin!
echo ".soroban" >> .gitignore
echo "*.secret" >> .gitignore

# Production için hardware wallet kullanın
# Testnet için ayrı key'ler kullanın
```

### 2. Amount Validation

```rust
// Kontrat içinde minimum/maximum kontrolleri
assert!(amount >= MIN_AMOUNT, "Amount too low");
assert!(amount <= MAX_AMOUNT, "Amount too high");
```

### 3. Rate Limiting

```rust
// Çok sık ödeme işlemini engelleyin
let time_since_last = current_time - subscription.last_payment;
assert!(time_since_last >= MIN_INTERVAL, "Too soon");
```

## 📈 Gelecek Özellikler

1. **Multi-token desteği**: USDC, EURC
2. **Discount codes**: Promosyon kodları
3. **Referral system**: Tavsiye sistemi
4. **Auto-renewal**: Otomatik yenileme
5. **Payment history**: Ödeme geçmişi
6. **Refund mechanism**: İade mekanizması
7. **Trial periods**: Deneme süreleri
8. **Family plans**: Aile planları

## 🆘 Sık Sorulan Sorular

**S: Ödeme zamanı geçtikten sonra ne olur?**
C: Kullanıcı `process_payment` fonksiyonunu çağırmalı. Otomatik ödeme için backend servisi gerekir.

**S: Abonelik iptal edersem param iade olur mu?**
C: Şu anki versiyonda hayır. İade mekanizması gelecek versiyonda eklenecek.

**S: Birden fazla servise abone olabilir miyim?**
C: Evet! Her servis için ayrı abonelik oluşturabilirsiniz.

**S: Fiyatlar değişebilir mi?**
C: Mevcut abonelikler sabit fiyatlıdır. Yeni abonelikler yeni fiyatlarla oluşturulur.
