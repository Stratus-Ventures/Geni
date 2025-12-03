# Environment Variables Setup Guide

## Required Environment Variables

### 1. Database Configuration

**`DATABASE_URL`** (Required)
- PostgreSQL connection string
- Format: `postgresql://username:password@host:port/database_name`
- **How to get it:**
  - Install PostgreSQL locally or use a hosted service (Supabase, Neon, AWS RDS)
  - Local example: `postgresql://postgres:postgres@localhost:5432/auth_db`
  - Create the database: `createdb auth_db`
  - Run the schema: `psql -d auth_db -f schema.sql`

### 2. Server Configuration

**`PORT`** (Optional, defaults to 3000)
- The port your auth service will run on
- Example: `3000`, `8080`, etc.

### 3. WebAuthn (Passkeys) Configuration

These have defaults for localhost development but should be changed for production:

**`RP_ORIGIN`** (Optional, defaults to `http://localhost:3000`)
- The origin of your application
- Production example: `https://yourdomain.com`

**`RP_ID`** (Optional, defaults to `localhost`)
- The relying party ID (usually your domain)
- Production example: `yourdomain.com`

**`RP_NAME`** (Optional, defaults to `Auth Service`)
- Display name for your app in passkey prompts
- Example: `Your App Name`

### 4. Google OAuth Configuration

**`GOOGLE_CLIENT_ID`** (Required)
**`GOOGLE_CLIENT_SECRET`** (Required)
**`GOOGLE_REDIRECT_URL`** (Required)

**How to get Google OAuth credentials:**

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project or select an existing one
3. Enable the Google+ API:
   - Go to "APIs & Services" > "Library"
   - Search for "Google+ API" and enable it
4. Create OAuth credentials:
   - Go to "APIs & Services" > "Credentials"
   - Click "Create Credentials" > "OAuth 2.0 Client ID"
   - Choose "Web application"
   - Add authorized redirect URIs:
     - Development: `http://localhost:3000/auth/google/callback`
     - Production: `https://yourdomain.com/auth/google/callback`
5. Copy the Client ID and Client Secret

**Example values:**
```
GOOGLE_CLIENT_ID=123456789-abcdefg.apps.googleusercontent.com
GOOGLE_CLIENT_SECRET=GOCSPX-abc123def456
GOOGLE_REDIRECT_URL=http://localhost:3000/auth/google/callback
```

### 5. Apple OAuth Configuration (Sign in with Apple)

**`APPLE_CLIENT_ID`** (Required)
**`APPLE_CLIENT_SECRET`** (Required)
**`APPLE_REDIRECT_URL`** (Required)

**How to get Apple OAuth credentials:**

1. **Enroll in Apple Developer Program** ($99/year)
   - Go to [Apple Developer](https://developer.apple.com/programs/)

2. **Create an App ID:**
   - Go to [Certificates, Identifiers & Profiles](https://developer.apple.com/account/resources/identifiers/list)
   - Click "+" to create a new identifier
   - Select "App IDs" and configure with "Sign in with Apple" capability

3. **Create a Services ID (Client ID):**
   - Go to Identifiers > "+"
   - Select "Services IDs"
   - Enter a description and identifier (e.g., `com.yourapp.service`)
   - Enable "Sign in with Apple"
   - Configure domains and redirect URLs:
     - Domains: `localhost` (dev), `yourdomain.com` (prod)
     - Redirect URLs:
       - `http://localhost:3000/auth/apple/callback` (dev)
       - `https://yourdomain.com/auth/apple/callback` (prod)

4. **Create a Private Key:**
   - Go to Keys > "+"
   - Enable "Sign in with Apple"
   - Download the .p8 private key file (keep it safe!)
   - Note the Key ID

5. **Generate the Client Secret (JWT):**
   - Apple requires a JWT signed with your private key as the client secret
   - The JWT must be regenerated periodically (max 6 months)
   - You'll need:
     - Team ID (found in top right of Apple Developer portal)
     - Services ID (Client ID from step 3)
     - Key ID (from step 4)
     - Private Key (.p8 file)

   **Generate JWT using a script or library:**
   ```javascript
   // Example using Node.js and jsonwebtoken
   const jwt = require('jsonwebtoken');
   const fs = require('fs');

   const privateKey = fs.readFileSync('AuthKey_XXXXXXXXXX.p8');
   const token = jwt.sign({}, privateKey, {
     algorithm: 'ES256',
     expiresIn: '180d',
     issuer: 'YOUR_TEAM_ID',
     subject: 'com.yourapp.service', // Your Services ID
     audience: 'https://appleid.apple.com',
     keyid: 'YOUR_KEY_ID'
   });

   console.log(token);
   ```

**Example values:**
```
APPLE_CLIENT_ID=com.yourapp.service
APPLE_CLIENT_SECRET=eyJhbGciOiJFUzI1NiIsImtpZCI6IkFCQ0QxMjM0NTYifQ.eyJpc3MiOiJERUY3ODkiLCJpYXQiOjE1MTYyMzkwMjIsImV4cCI6MTUxNjMyNTQyMiwiYXVkIjoiaHR0cHM6Ly9hcHBsZWlkLmFwcGxlLmNvbSIsInN1YiI6ImNvbS5leGFtcGxlLmFwcCJ9.signature
APPLE_REDIRECT_URL=http://localhost:3000/auth/apple/callback
```

**Note:** The Apple client secret (JWT) expires and needs to be regenerated. Consider automating this in production.

### 6. Email Service (Resend)

**`RESEND_API_KEY`** (Required)

**How to get Resend API key:**

1. Go to [Resend](https://resend.com/)
2. Sign up for an account (free tier available)
3. Go to [API Keys](https://resend.com/api-keys)
4. Click "Create API Key"
5. Give it a name and copy the key (starts with `re_`)

**Example value:**
```
RESEND_API_KEY=re_123abc456def789ghi
```

## Quick Start

1. Copy the example file:
   ```bash
   cp .env.example .env
   ```

2. Fill in your actual values in `.env`

3. Run the service:
   ```bash
   cargo run
   ```

## Development Tips

- For local development, you can use placeholder values for OAuth if you're not testing those features
- WebAuthn (passkeys) will work with the default localhost settings
- Make sure your PostgreSQL database is running before starting the service
- Never commit your `.env` file to version control (it's in `.gitignore`)

## Production Considerations

- Use HTTPS for all redirect URLs in production
- Rotate API keys and secrets regularly
- Store secrets in a secure vault (AWS Secrets Manager, HashiCorp Vault, etc.)
- Apple client secret (JWT) expires - implement automatic regeneration
- Use environment-specific `.env` files
