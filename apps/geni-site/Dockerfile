FROM node:20-alpine

WORKDIR /app

# Copy package files
COPY package.json yarn.lock ./

# Install dependencies
RUN yarn install --frozen-lockfile

# Copy source code
COPY . .

# Generate database types and build
RUN yarn build

# Expose port 4000
EXPOSE 4000

# Start the application
CMD ["node", "build"]