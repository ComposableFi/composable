# PoC Vault and Liquidity Rebalancing Process

---

Following the closure of our L1 vault, Composable has made preparations to commence the PoC ensuring liquidity on the respective L2 for Polygon and Arbitrum. 

As bridges do not have contract interaction functionality as yet, Composable built an automated process for facilitating liquidity transfer. At a high level, this 3 step process includes:

**Preparing Liquidity**

1. Moving USDC and wETH out of the L1 liquidity vault to a secured wallet
2. USDC and wETH are then moved from the secured wallet to a secured wallet on each of Polygon and Arbitrum. Roughly 25% of the deposited funds were stored in the secured wallet to ensure the ability to rebalance throughout the duration of the POC. 68 wETH and 198k USDC were stored in the L1 vault located at: `0xef4439f0fAe7DB0B5ce88C155fc6af50F1b38728`
3. USDC and wETH are then send to the respective L2 Vaults on each of Polygon and Arbitrum 

The detailed process and the respective wallets to ensure transparency are outlined below: 

### 1. USDC and wETH out of the L1 liquidity vault: 

- USDC: [https://etherscan.io/tx/0xcce0a86928808ff4ef0910f51d37dc3e86892cebc159a2d23d1ecbffa79ae8af](https://etherscan.io/tx/0xcce0a86928808ff4ef0910f51d37dc3e86892cebc159a2d23d1ecbffa79ae8af)
- wETH: [https://etherscan.io/tx/0x19d9baebcd81f7d21253917a25d8c2a6b395b04c52e3c95353c36767439c7df2](https://etherscan.io/tx/0x19d9baebcd81f7d21253917a25d8c2a6b395b04c52e3c95353c36767439c7df2)

### 2. Movement to Polygon and Arbitrum wallets

* **Arbitrum - 183k USDC and 60 wETH**
    
    * ​[10k USDC - part 1](https://etherscan.io/tx/0x27ed4db099460b376a84a29de0379f4ee7da713d4aaeb985ad9f5b775d1e5bf1)​
        
    * ​[10k USDC - part 2](https://arbiscan.io/tx/0x1ba71e0d68866c147e2db945805dd25fc15914342d6c509cf7448b212b03a883)
        
    * ​[30k USDC - part 1](https://etherscan.io/tx/0x13e6b79bb44070b0d5a054a835140de7d13f581fa4fe941fce411bee05c07463) ​
        
    * ​[30k USDC - part 2](https://arbiscan.io/tx/0xa5ac6b7012ae600d15337169594a50ca51a9335090214e28b13189b126456a05)​
        
    * ​[50k USDC - part 1](https://etherscan.io/tx/0x65a85b9502f04446ac60f1e568f08fc94cbf388fe5324cabb2498047825c0fa0) ​
        
    * ​[50k USDC - part 2](https://arbiscan.io/tx/0xf1a6dcd0039d74ff78c6ea6b2382ff4592df0dd0aafecb6806287a9bc5422b8e)​
        
    * ​[50k USDC - part 1](https://etherscan.io/tx/0x1dd78dca2b880d1cca576dad89c85baefdb04a954cb1b0d0e263bc547b509177) ​
        
    * ​[50k USDC - part 2](https://arbiscan.io/tx/0x5be0c93219df704c3e68931fa3026cc0bbd13ad4353963863cb3595ba8db08c4)​
        
    * ​[43k USDC - part 1](https://etherscan.io/tx/0x87ced719edd828a21c11eaad979ca0b9e71f8d18d5c291d94379c3bbd848c955) ​
        
    * ​[43k USDC - part 2](https://arbiscan.io/tx/0xf2ccffe3cbc882e2f91a2481b74c227b0256e7536c63d3be7f887d18d40de391)​
        
        * Transfer USDC to the owner wallet: [https://arbiscan.io/tx/0xeaf93e7a5d75cc5fbbd9c1cec6c773bcbd0846cade8c97ae8141e947ee385087](https://arbiscan.io/tx/0xeaf93e7a5d75cc5fbbd9c1cec6c773bcbd0846cade8c97ae8141e947ee385087) and [https://arbiscan.io/tx/0xb967915f4cfe1e5c3530e355ab5a1827bf0cc685ee75b7f23c5a142be43b9ba6](https://arbiscan.io/tx/0xb967915f4cfe1e5c3530e355ab5a1827bf0cc685ee75b7f23c5a142be43b9ba6)​
            
        
    * ​[10 WETH - part 1](https://etherscan.io/tx/0x67e21d3757083183bf283c279e90c2c87d29670d01b8223318b8afa922b73dc4)
        
    * ​[10 WETH - part 2](https://arbiscan.io/tx/0xf2ccffe3cbc882e2f91a2481b74c227b0256e7536c63d3be7f887d18d40de391) ​
        
    * ​[20 WETH - part 1](https://etherscan.io/tx/0xb017a52097c02662a98c19c856403dddb97d93aeab45fdc9104b63b49620397f) ​
        
    * ​[20 WETH - part 2](https://arbiscan.io/tx/0x2e00a5165c7df490d7e6c2a63ee8ce2c96fd164d2e469b3303ecfcfa9ff9e8cf) ​
        
    * ​[30 WETH - part 1](https://etherscan.io/tx/0x9e1a69cf2097ee2ec1498815a6f11bf2ff91bd400371afe358a3c98b400bb2e9) ​
        
    * ​[30 WETH - part 2](https://arbiscan.io/tx/0x12da1e3deef032547b990913999145f456b0896e8422cf6cbdcc83c85a74cb1d) ​
        
        * Transfer WETH to the owner wallet: [https://arbiscan.io/tx/0x4e12378ca744eb46899010048bf57f946f8f8e310d8f8db7d9965f6ecf816cd0](https://arbiscan.io/tx/0x4e12378ca744eb46899010048bf57f946f8f8e310d8f8db7d9965f6ecf816cd0) and [https://arbiscan.io/tx/0x93ea4ac0776d3eb499b2f6d2bd543274291dba58d83a761a99e5b21e07042d34](https://arbiscan.io/tx/0x93ea4ac0776d3eb499b2f6d2bd543274291dba58d83a761a99e5b21e07042d34)
            
        
    * **Polygon - 183k USDC and 60 wETH**
        
        * ​[1k USDC - part 1](https://etherscan.io/tx/0xa46101814c7117ec67cfbbe32a8ecb28405ae50a2707b306499d10ad35a81ebb)​
            
        * ​[1k USDC - part 2](https://polygonscan.com/tx/0xbd9e2d9b7fdf586a058b513355a90c68d46a525ac6d061b92270dd0f782c08b9) ​
            
        * ​[49k USDC - part 1](https://etherscan.io/tx/0xd2ac7db25f9a7ee3cc657ce6799e11eb84645010af384aee3210b51a764dc617)​
            
        * ​[49k USDC - part 2](https://polygonscan.com/tx/0x41b52c98af9a1e2c3d82141211a2e74046a415424f0c20c98e8ca4bd11feeb65)​
            
        * ​[50k USDC - part 1](https://etherscan.io/tx/0x046845b00f84d16893635acfe4a3daca0bb3916438f236dfeb4a5e756df46b7d)​
            
        * ​[50k USDC - part 2](https://polygonscan.com/tx/0x05f5ffd14f14d1e3220ab77f7467fe7060d7d015967f691dd2313292567cd9ef)​
            
        * ​[83k USDC - part 1](https://etherscan.io/tx/0x11e892424e42d049282f290bc336ee15b84909bd12216ed47e38c560f28811bc)​
            
        * ​[83k USDC - part 2](https://polygonscan.com/tx/0x68b7c59ce663e68767a514057da3ed048d081334314485b0e2333ca0bd1a6955)​
            
            * Transfer USDC to the owner wallet: [https://polygonscan.com/tx/0xe93e5a54e13a332d7a4b3e9451fb496209bc160be4169851d61bd688aabb2c12](https://polygonscan.com/tx/0xe93e5a54e13a332d7a4b3e9451fb496209bc160be4169851d61bd688aabb2c12); [https://polygonscan.com/tx/0x4389583e315297e07b1ea117a51bcc9ca835db965d4b102e853120ad06a9c704](https://polygonscan.com/tx/0x4389583e315297e07b1ea117a51bcc9ca835db965d4b102e853120ad06a9c704); [https://polygonscan.com/tx/0x8b8d71bec36da1b767d751f187b1a6dc447676b6d8e1534c94166ee73f7d39ee](https://polygonscan.com/tx/0x8b8d71bec36da1b767d751f187b1a6dc447676b6d8e1534c94166ee73f7d39ee)​
                
            
        * ​[Unwrap 1 WETH](https://etherscan.io/tx/0xecaca55d886e567ccfae5eea67cf92d056b92cfbc174d872da27d2164ec932b9)​
            
        * ​[Unwrap 10 WETH](https://etherscan.io/tx/0x3c13cdba8bb89bba1b75c453901ceca1b679fa94d44db9a7fba66ff1136a17ee)​
            
        * ​[Unwrap 19 ETH](https://etherscan.io/tx/0x30a8ee4a55adcfdca1b1715f77667e18d09c7791c9c642931e8eed743252daeb)​
            
        * ​[Unwrap 30 ETH](https://etherscan.io/tx/0x31acb14195143a2e774c30531bf5baa1b84dac7e2df782cc65ffa4e5a7c29eb8)​
            
        * ​[1 ETH - part 1](https://etherscan.io/tx/0xcb5c030682650d8f0ec4684bea77bd5b0743025f481e684c0b8ddd584899699e)​
            
        * ​[1 ETH - part 2](https://polygonscan.com/tx/0xfbaf64ff8fd09b2a85c6d6590303ca6a0bbabe2b7e0663272855ca3ad55d2394) ​
            
        * ​[30 ETH - part 1](https://etherscan.io/tx/0x6fcb1cd550a32b59c85a0ed270d6e0ca121f828ec9188c5f4a6fd2ba1e583491) ​
            
        * ​[30 ETH - part 2](https://polygonscan.com/tx/0x9af5d2887cbbd741c3eb6354429928ec841967716e604da7315fdcf43b7c3cf3)​
            
        * ​[29 ETH - part 1](https://etherscan.io/tx/0xec1ac223ee2a78f9a8903ffe0f45c2e45b92339daf47b1a9d3df70346165bf1c)​
            
        * ​[29 ETH - part 2](https://polygonscan.com/tx/0xfc4ab5bc868609a20924836b7c8880e9d87a683dfdb5facb63293412412db1d5)​
            
        * Transfer WETH to the owner wallet: [https://polygonscan.com/tx/0xccd2b3a89899796b3f0ff560725737e3b75b297c57a2503c1bd3890b023b77b4](https://polygonscan.com/tx/0xccd2b3a89899796b3f0ff560725737e3b75b297c57a2503c1bd3890b023b77b4); [https://polygonscan.com/tx/0xe90ce1ea4ae08d5cf043c3d23670b44f141f56f4f387df1a8f09c574ffea84d2](https://polygonscan.com/tx/0xe90ce1ea4ae08d5cf043c3d23670b44f141f56f4f387df1a8f09c574ffea84d2)​

### 3. USDC and wETH are then send to the respective L2 Vaults on each of Polygon and Arbitrum 


**i.** Move from Mainnet(L1) wallet to Mainnet(L1) L2Vault (183k USDC, 60WETH)

USDC: [https://etherscan.io/tx/0xbff5f0f2d3fc7db8ce5508783890cce804b73926d7dd3053a56366b5821cd12d](https://etherscan.io/tx/0xbff5f0f2d3fc7db8ce5508783890cce804b73926d7dd3053a56366b5821cd12d)​

WETH: [https://etherscan.io/tx/0x4c20b9175e038cb49b2f614921615b59c172e048cbab54d38e0557517e7fc2b5](https://etherscan.io/tx/0x4c20b9175e038cb49b2f614921615b59c172e048cbab54d38e0557517e7fc2b5)​

ii, Move from Arbitrum wallet to Arbitrum L2Vault (183k USDC, 60WETH)

USDC: [https://arbiscan.io/tx/0x6874bf7313f0bdccd86a50460a906c9d0f531105d3513701bc0781e26f028d78](https://arbiscan.io/tx/0x6874bf7313f0bdccd86a50460a906c9d0f531105d3513701bc0781e26f028d78) and [https://arbiscan.io/tx/0xfab0ee44b1c62ff35760fa137ed86c736eb0383392a36fac57d59569c78f3bdc](https://arbiscan.io/tx/0xfab0ee44b1c62ff35760fa137ed86c736eb0383392a36fac57d59569c78f3bdc)​

WETH: [https://arbiscan.io/tx/0x2b210da94af223477518e4e6e3d3e5f542f3d5288961aef2c4c500355145ac4c](https://arbiscan.io/tx/0x2b210da94af223477518e4e6e3d3e5f542f3d5288961aef2c4c500355145ac4c) and [https://arbiscan.io/tx/0x00cd1625947697a82593651738b77ec36a74ecb57d2451766f38a4c4e919f045](https://arbiscan.io/tx/0x00cd1625947697a82593651738b77ec36a74ecb57d2451766f38a4c4e919f045) and [https://arbiscan.io/tx/0xae348976fac74fde7c4ca1341ccede18f2b5688f91bd225a44ae4456ec3cc68e](https://arbiscan.io/tx/0xae348976fac74fde7c4ca1341ccede18f2b5688f91bd225a44ae4456ec3cc68e)​

iii. Move from Polygon wallet to Polygon L2Vault (183k USDC, 60WETH)

USDC: [https://polygonscan.com/tx/0x9c759b8e8e82178c6233c3521a749c48b8fa3e14a1457077673bf7b5c9667099](https://polygonscan.com/tx/0x9c759b8e8e82178c6233c3521a749c48b8fa3e14a1457077673bf7b5c9667099) and [https://polygonscan.com/tx/0x3b1d08264d87090439a486c922eb8c1ad5cf3414d233b9f02b9102f983bc1009](https://polygonscan.com/tx/0x3b1d08264d87090439a486c922eb8c1ad5cf3414d233b9f02b9102f983bc1009)​

WETH: [https://polygonscan.com/tx/0xd97bf474fd2c0dc3ea8748bbb334804ac24372a3899cbec5934aa920493fc2dd](https://polygonscan.com/tx/0xd97bf474fd2c0dc3ea8748bbb334804ac24372a3899cbec5934aa920493fc2dd) and [https://polygonscan.com/tx/0x2c732b1f399bcf17e66d1e9343c3a970fc80161b9d476bcc8a21e9d47ddc9c57](https://polygonscan.com/tx/0x2c732b1f399bcf17e66d1e9343c3a970fc80161b9d476bcc8a21e9d47ddc9c57) and [https://polygonscan.com/tx/0x5354f181ef4474e8c97beb6442c88c87a79f92a8546e8986f049b1728b80035b](https://polygonscan.com/tx/0x5354f181ef4474e8c97beb6442c88c87a79f92a8546e8986f049b1728b80035b)

---

### Liquidity Rebalancing Event

In order to ensure there is sufficient liquidity in each of the L2 vaults to facilitate the operation of the PoC, Mosaic has defined a process to trigger a rebalancing event to replenish liquidity. At a high level the process is as follows:

* Once either of the respective L2 vaults on Polygon or Arbitrum reaches a certain threshold, this will trigger a rebalancing event
    
* Mosaic will shift equal amounts of USDC and wETH to the other vault
    
* Composable will also announce on it's official channels once a rebalancing event has occurred with the respective transaction hash
    