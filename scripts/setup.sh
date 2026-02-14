#!/bin/bash

# Ethereum Boilerplate Rust - Setup Script
echo "🚀 Setting up Ethereum Boilerplate Rust Edition..."

# Check if required tools are installed
check_tool() {
    if ! command -v $1 &> /dev/null; then
        echo "❌ $1 is not installed. Please install $1 first."
        exit 1
    else
        echo "✅ $1 is installed"
    fi
}

echo "🔍 Checking required tools..."
check_tool "rustc"
check_tool "cargo"
check_tool "node"
check_tool "npm"
check_tool "psql"

# Setup Rust backend
echo "🦀 Setting up Rust backend..."
cd server
cargo build
echo "✅ Rust backend built successfully"

# Setup frontend
echo "⚛️ Setting up frontend..."
cd ../frontend
npm install
echo "✅ Frontend dependencies installed"

# Setup smart contracts
echo "📜 Setting up smart contracts..."
cd ../smart-contracts
npm install
echo "✅ Smart contract dependencies installed"

# Setup database (optional)
echo "🗄️ Setting up database..."
read -p "Do you want to setup PostgreSQL database? (y/n): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "Creating database..."
    createdb ethereum_boilerplate
    echo "✅ Database created"
fi

# Create .env files
echo "📝 Creating environment files..."
cd ..

if [ ! -f .env ]; then
    cp .env.example .env
    echo "✅ Created .env file. Please update with your configuration."
fi

if [ ! -f frontend/.env.local ]; then
    cp .env.example frontend/.env.local
    echo "✅ Created frontend/.env.local file."
fi

if [ ! -f smart-contracts/.env ]; then
    cp .env.example smart-contracts/.env
    echo "✅ Created smart-contracts/.env file."
fi

echo "🎉 Setup completed!"
echo ""
echo "📋 Next steps:"
echo "1. Update .env files with your API keys and configuration"
echo "2. Start PostgreSQL if you're using database features"
echo "3. Run 'cargo run --bin server' to start the backend"
echo "4. Run 'npm run dev' in frontend/ to start the frontend"
echo "5. Run 'npm run node' in smart-contracts/ to start local blockchain"
echo ""
echo "📚 For more information, check the README.md file"
