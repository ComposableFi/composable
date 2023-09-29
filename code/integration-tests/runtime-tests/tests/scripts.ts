import { queryAllBalanceOnPicasso} from "./utils/ibcUtils";
import { picassoEndpoint, picassoFeeAddress} from "./utils/constants";
import {initializeApi} from "./utils/apiClient";
const main = async () =>{
  const api = await initializeApi(picassoEndpoint);
  const preBalances = await queryAllBalanceOnPicasso(api, [1,4,6,130], picassoFeeAddress);
  console.log(preBalances['1']);
}

main().then(() => {
  process.exit(0);
});