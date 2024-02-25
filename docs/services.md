# Services

- Authentication
    - JWT
    - SignIn with google
    - generateUsername: must be unique
- Compliance
     - KYC
    - SmileId
- Notification
    - Termi :  Send to phone
    - SendGrid
    - Letter: Send to email
- Fiat Wallets
    - Create Virtual Bank Account
        - pass credential and await approval
- Crypto Wallet
    - Create wallet: choose blockchain of choice CUSD/USDT
    - Save user passPhrase
- Transactions
    - One collector for transactions
    - State of transaction
    - Type of transaction
    - Both Fiat and crypto trans impl similar interface
    - Fetch, Sort, Filter through transactions
    - Generate nice pdf for statement
    - Select and ability to filter
- FX
    - Pull data from FX web apis
    - Use data to estimate and calculate selling price and buying price
      - NGN/USD
      - USD/NGN
      - NGN/EURO
      - NGN/GPB
      - NGN/BTC
      - NGN/ETH
      - NGN/CUSD
- Payments
  - Receive Funds via username
    - Accepts crypto funds 
    - Company wallet accepts the funds then deposit it into user account
  - Deposit Fiat
    - Debit customer card to fund virtual account
    - Direct cash transfer to bank account
      - Webhook


## Others

- Logging: Tracing
- Database:
  - Diesel setup
  - Create Tables with properties
