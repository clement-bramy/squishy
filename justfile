# Run all pre-release checks
check-all:
    @echo "🔍 Running formatting check..."
    cargo fmt --check
    @echo "📎 Running clippy..."
    cargo clippy -- -D warnings
    @echo "🧪 Running tests..."
    cargo test
    @echo "✅ All checks passed!"

# Run checks and release if they pass
release: check-all
    @echo "🚀 Running cargo release..."
    cargo release patch --execute

# Fix formatting issues
fix:
    cargo fmt
    cargo clippy --fix --allow-dirty
