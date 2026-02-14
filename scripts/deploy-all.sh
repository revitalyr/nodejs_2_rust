#!/bin/bash

# Ethereum Boilerplate Rust - Full Deployment Script
echo "🚀 Starting full deployment..."

# Function to check if command succeeded
check_success() {
    if [ $? -eq 0 ]; then
        echo "✅ $1 completed successfully"
    else
        echo "❌ $1 failed"
        exit 1
    fi
}

# Deploy smart contracts
echo "📜 Deploying smart contracts..."
cd smart-contracts
npm run compile
check_success "Contract compilation"

# Choose network based on environment
NETWORK=${1:-localhost}
echo "🌐 Deploying to network: $NETWORK"

if [ "$NETWORK" = "localhost" ]; then
    echo "🔧 Starting local Hardhat node in background..."
    npm run node &
    HARDHAT_PID=$!
    sleep 5  # Wait for node to start
    
    npm run deploy:local
    check_success "Local contract deployment"
    
    kill $HARDHAT_PID 2>/dev/null
else
    npm run deploy:$NETWORK
    check_success "$NETWORK contract deployment"
fi

# Copy deployment info to frontend config
cd ..
if [ -f "smart-contracts/deployments/$NETWORK.json" ]; then
    echo "📋 Updating frontend contract configuration..."
    cp smart-contracts/deployments/$NETWORK.json config/contracts.json
    echo "✅ Contract configuration updated"
fi

# Build and start backend
echo "🦀 Building and starting backend..."
cd server
cargo build --release
check_success "Backend build"

# Start backend in background
cargo run --release &
BACKEND_PID=$!
echo "🚀 Backend started (PID: $BACKEND_PID)"

# Build and start frontend
echo "⚛️ Building and starting frontend..."
cd ../frontend
npm run build
check_success "Frontend build"

npm start &
FRONTEND_PID=$!
echo "🎨 Frontend started (PID: $FRONTEND_PID)"

echo ""
echo "🎉 Full deployment completed!"
echo ""
echo "📊 Services running:"
echo "- Backend: http://localhost:3000 (PID: $BACKEND_PID)"
echo "- Frontend: http://localhost:3001 (PID: $FRONTEND_PID)"
echo "- Network: $NETWORK"
echo ""
echo "🛑 To stop all services, run: kill $BACKEND_PID $FRONTEND_PID"
echo ""
echo "📚 Check the logs for each service in their respective terminals"
