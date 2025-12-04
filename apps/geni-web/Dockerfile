FROM node:latest

WORKDIR /app

# 1. Install pnpm globally using npm (more reliable than corepack)
RUN npm install -g pnpm

# 2. Copy ONLY package files first (to cache dependencies)
COPY package.json pnpm-lock.yaml ./

# 3. Install dependencies
RUN pnpm install --frozen-lockfile

# 4. Copy the rest of the source code
COPY . .

# 5. Build the app
RUN pnpm run build

# Set host to allow external access
EXPOSE ${PORT}

CMD ["node", "build"]