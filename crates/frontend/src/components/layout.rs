use leptos::*;
use web_sys::console;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="header">
            <div class="header-content">
                <div class="logo">
                    <h1>"🦀 ETH Boilerplate"</h1>
                </div>
                <nav class="nav">
                    <a href="/" class="nav-link">"Home"</a>
                    <a href="/balances" class="nav-link">"Balances"</a>
                    <a href="/transactions" class="nav-link">"Transactions"</a>
                    <a href="/contracts" class="nav-link">"Contracts"</a>
                </nav>
                <WalletConnector/>
            </div>
        </header>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="footer-content">
                <p>"Built with Rust 🦀 | Leptos ⚛️ | ethers-rs 🌐"</p>
                <div class="footer-links">
                    <a href="https://github.com/ethereum-boilerplate-rust" target="_blank">
                        "GitHub"
                    </a>
                    <a href="https://docs.rs/ethers/" target="_blank">
                        "Docs"
                    </a>
                </div>
            </div>
        </footer>
    }
}

#[component]
pub fn LoadingSpinner() -> impl IntoView {
    view! {
        <div class="loading-container">
            <div class="spinner"></div>
            <p>"Loading..."</p>
        </div>
    }
}

#[component]
pub fn ErrorMessage(message: String) -> impl IntoView {
    view! {
        <div class="error-container">
            <div class="error-icon">"⚠️"</div>
            <p>{message}</p>
        </div>
    }
}

#[component]
pub fn SuccessMessage(message: String) -> impl IntoView {
    view! {
        <div class="success-container">
            <div class="success-icon">"✅"</div>
            <p>{message}</p>
        </div>
    }
}
