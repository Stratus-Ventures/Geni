FROM node:latest
LABEL authors="jasoncoawette"

WORKDIR /app

# Enable pnpm
RUN corepack enable

# Install dependencies and build the project
RUN pnpm install --frozen-lockfile

# Copy all local files to the container
COPY . .

RUN pnpm run build

EXPOSE ${PORT}

# Start the Node server
CMD ["node", "build"]