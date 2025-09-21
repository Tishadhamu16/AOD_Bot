use ethers::prelude::abigen;

// abigen generates Rust bindings for our smart contract ABIs at compile time
// uniswap V2 router is a smart contract that acts as an interface for users to interact with uniswap V2 
abigen!(
    IUniswapV2Router02,
    r#"[
        function getAmountsOut(uint amountIn, address[] memory path) public view returns (uint[] memory amounts)
    ]"#,
);