use std::collections::HashSet;

use crate::common::{
	api_wrap, extract_account, multi_account_id, Amount, Batch, Canonical, CommonError, Raw,
};
use chrono::{Duration, NaiveDate};
use composable_traits::vesting::{VestingScheduleInfo, VestingWindow};
use primitives::currency::CurrencyId;
use sp_core::crypto::Ss58Codec;
use sp_runtime::{codec::Compact, AccountId32};
use ss58_registry::Ss58AddressFormatRegistry;
use substrate_api_client::{
	compose_call, compose_extrinsic, rpc::WsRpcClient, Api, AssetTipExtrinsicParams,
	GenericAddress, XtStatus,
};

const INFRASTRUCTURE_PROVIDERS: &[(&str, Amount<Raw>)] = &[
	("5xD9TpbabV6VdC2F9RL5yQA61DPzWfWn1U6b76ckZBeQL2vo", Amount::new(1_000_000)),
	("5wyRiyzsTWvXcb7BpHYjDs7GgK3VpbccRTdonv2Wvow7GAkg", Amount::new(2_962_963)),
	("5CGCdbYh9KwmSM8NYPCRuz4tUqnbfAUtNy4Ei5Rg2qujatqR", Amount::new(1_000_000)),
	("5zEddnhFgz8yyAXwMDew9SMNQveGmcEP73CThfYh6DfkQXMH", Amount::new(8_000_000)),
	("5zEddnhFgz8yyAXwMDew9SMNQveGmcEP73CThfYh6DfkQXMH", Amount::new(2_000_000)),
	("16A5r6WjR2DxE5iezedrEEf4aSrKkNMwF1Py8ir52pk1Z5oN", Amount::new(1_481_481)),
	("DzepVXv9Z6RGKTJUWj6jECqaRh7gpYc7sGSYdG6Yzoj2dWm", Amount::new(2_518_519)),
	("162LiM9SQSPUBQW8ZUivmg8oTE4DHKDDrNpf7TUDg4RdVQbj", Amount::new(2_000_000)),
	("5ECzzYfSwuSd64mYu2Vn42NnqVCm2o9HtGi6Q7NLuBqewZDd", Amount::new(2_000_000)),
	("5uPfUKgKc76LN2t6Cj7BVAnY5trNm8YK23Cgn4kwWWoYu7AL", Amount::new(2_000_000)),
	("5D215FWsfUqiBUYbtioxcb6mCbY5AFcUimPhhiF9xHieoTs1", Amount::new(2_000_000)),
	("5HT2WdLNgm6vDa5Xdx6dyPAGL2Qtsy4f6HGkpnGjtC1u4Ms4", Amount::new(1_000_000)),
	("5DLTAmm2tcm4zGX1r9wGae5LCC5L2JHqXu2resqg5e3PgALj", Amount::new(8_000_000)),
	("5DLTAmm2tcm4zGX1r9wGae5LCC5L2JHqXu2resqg5e3PgALj", Amount::new(4_000_000)),
	("5FHPEDBTdfyqX4MnLwSJ7F2mp9e9Uhsx8fshxSL6KBwjSTWD", Amount::new(1_000_000)),
	("5vSR8xyCVUtD2qiHEPiG9owNnFxpUQfzEfK8AAuAYAW9aGoR", Amount::new(1_000_000)),
	("5vSR8xyCVUtD2qiHEPiG9owNnFxpUQfzEfK8AAuAYAW9aGoR", Amount::new(4_000_000)),
	("5yhcepLAujpJcQfLMkZdLE4tPLBjyqaSCiQq1NvYak6D9rD1", Amount::new(2_000_000)),
	("1BkvLWCxkvrmiTjogjRgUpfDV7TCRYRkPamAyHgxpvZBrFG", Amount::new(2_000_000)),
	("1BkvLWCxkvrmiTjogjRgUpfDV7TCRYRkPamAyHgxpvZBrFG", Amount::new(1_000_000)),
	("5Cevx2zUE4PcqtF1wMHr1msDWvXkZuFtEfG5crEp3SYp5bNF", Amount::new(2_000_000)),
	("5Cevx2zUE4PcqtF1wMHr1msDWvXkZuFtEfG5crEp3SYp5bNF", Amount::new(1_000_000)),
	("5HYdhBoTQR7fdqFLeZuMzHn2GktSUyZyKrr8spQb7i94etru", Amount::new(2_888_889)),
	("5tmsbdYgTBwQMZ1QbsN1STUV58DeNSW9nQZSHdjd46ezeArM", Amount::new(1_000_000)),
	("5vdQNCfbwinB5jYp2EfxUPpj3Cq1nJMDWnJUXcKfYs2LYzuh", Amount::new(2_456_140)),
	("15tHCtqr2ct42aaN4oiw2yVfiPUutGLzKMcjnwX5Y5tskwMx", Amount::new(2_105_263)),
	("5xHPbqATyTk9hkFwjYP8duX9Qy3T1PRUtF6WJdM9GPoAyZpf", Amount::new(1_500_000)),
	("5xHPbqATyTk9hkFwjYP8duX9Qy3T1PRUtF6WJdM9GPoAyZpf", Amount::new(1_000_000)),
	("15nY6N7RiWtVhEAX8jNYWPgNEiFrATw93SLsufdBw2XUtAAo", Amount::new(4_108_333)),
	("5yP5uDcTmQMPsgUPyB86j3pxmDo27mrWbiJCjd2cBrkfopAY", Amount::new(3_111_111)),
];

const INVESTORS: &[(&str, Amount<Raw>)] = &[
	("5v5AKbVaHzCx3p56gdnXHczQehRZv6s32rUT2Es2jVfzne9N", Amount::new(25000000)),
	("5v5AKbVaHzCx3p56gdnXHczQehRZv6s32rUT2Es2jVfzne9N", Amount::new(13333333)),
	("5woSpAuR2waSjktAcSMGtAi4n43fN6V4issAMJ8N168KZfqW", Amount::new(100000000)),
	("133SJBiEn4BFZwCo8xvPj4rfs28dsGb4SsxHySx8AsycFUAk", Amount::new(24000000)),
	("5wAUXbMNm3D6ZaEUxg9bMRDuCvFqEzBUNQUz143sy2f6LhFN", Amount::new(12500000)),
	("FySkBAgA3V44AGzR4dWEZ9poYsKKXcyrmS2ksqEaFEi8TPm", Amount::new(5000000)),
	("5F3rjNwzo21nb5aJcEYQenBmKss7GbJJgkUx7Rx6ymymdYME", Amount::new(1250000)),
	("13pPDTFCfn97r9dW2yDZbLfAiNRqr3jprzciFS9bXsWTPu48", Amount::new(8333333)),
	("5FWAXUmU6z1RaCkFPWgCZ1fz4CRanurQW6AwrvajK3kUNE5R", Amount::new(125000)),
	("5FTbTJ1Yq5hcAdLxZsqxgs4vEZRHqZrbjN6yCWeuvTvM72tK", Amount::new(62500)),
	("5DwExBiKbQAPwo2ZvvodeVwPJ13zLf3ErWvX8fM2xKM8pupD", Amount::new(100000)),
	("5w5VXjbjcuBzkc4m6JvY8gsF8ygFCUZx3RDUMu3wEYq5csLN", Amount::new(125000)),
	("5EjiCJfaUvKnNmaKBa5PcVfcWwBqgdCRXrjNBsjfomG6Nvvd", Amount::new(62500)),
	("5GMv2qqqKUpRRTsXHCvDrkGK83XYtB7EwjcYfStbgqFzSuhu", Amount::new(62500)),
	("5GBtD846Pv6Pz31QkHzDEdb6oFiveYgk8yyc4KoSnxzizAuS", Amount::new(166666)),
	("5GKSPzJFj5xCp8EFrYH2ujGv3u6sfNFsauMbk3oQ6kzwb8vk", Amount::new(62500)),
	("5E7HsPkHXvbADZ9MoaA2pEfZ8xJGrAj93sdHDQb9waxxvu2n", Amount::new(62500)),
	("5EREVtZKauHfkSsDiSt5YoFqZ7TPbWmnPqsQDRsDDq4SS5eB", Amount::new(62500)),
	("5FxVfBoM6H9PKYDti5jg5fDHa3HfToDQnqJfhjY37AJQY7os", Amount::new(62500)),
	("5GpJKGCSqcYodjRo3dUSJp4y2dZshmHEGHuoeri7FGX2Wict", Amount::new(62500)),
	("5CUTaZHm16VkRnmSnszpUP7kwkvszSGYPeX6rBmcjCVCQ3Cw", Amount::new(125000)),
	("5FC38YuEdP6tr9YvXwfHgYbvzLkKkN4LDYrYFQkbpW7FEkEY", Amount::new(62500)),
	("5GEjMxaUadsae5mt4vbsf1Wv8SM9oh4JY25zQt1xSaLCWgxf", Amount::new(62500)),
	("5Fv1tr96VtQ3R7ZtNXb9CMaByb51fdwXbyu9SEHko9p8r8wX", Amount::new(62500)),
	("5G6G8LffmbvukLJXV9dXh6KK3au2AnkKdmScVMFwqszmdk1t", Amount::new(62500)),
	("5EHu1sJp38xYmokHbHYTCg3ZP5Zq1RssohEwBEJmaEX27gmL", Amount::new(250000)),
	("5FCP2hEY6FYPU7da26cnBwX1fabqqyF8AuLG5EuQaYgN7fB2", Amount::new(125000)),
	("5F1HFreR9q8w6go1NU9Api4EL8yACWuPttpytrCvzkGk6n7q", Amount::new(62500)),
	("5HThhNn8mFRqWtffZA8TX69rURMd19s444kv1pUDWpW1eEPL", Amount::new(9375)),
	("5HTWmi3Zr58qgVAvmXH7afFXNvUdvkQ2ujk6jUsSRLks2v74", Amount::new(18750)),
	("5GRKZm1KetaXZQfro8eLeenLqnTVRLfoScRHyPchxvTvKu6k", Amount::new(9375)),
	("5HTSgN7i1sWfwGJkYpSfGJQsSihUK1Wt6aRqtgoU4oHfVx67", Amount::new(18750)),
	("5Gc2BeETQ3nM7MwrBBGpJfA5HcxYSVQad83wLgPySfm17E8y", Amount::new(539063)),
	("5CUnWARbAr9RzuZcJWuT781UKsGw8rVAvtqUvZeDs18tbETK", Amount::new(164063)),
	("5EEfpoguDnvMPc91vP2uhKC1Bc6RpStKS3sdsEndFawcrxsc", Amount::new(93750)),
	("5FRA964Yn4Mqa2nbZJr9jrrHrWQVNahsRpFFcpZXSR361E7b", Amount::new(646874)),
	("5FLpmeNmcRA5caH1aBVeA2Fei9u4jBnLrooZb77dfpeWAW9j", Amount::new(62500)),
	("5EvoZ9CoMLEdzfh3G6XNRxUN4aiEyD8JiSR2oUDeojGLGYhe", Amount::new(187500)),
	("5D5aH8MQjo2Z55dczMGZXVy79r1dnTCqSwhQXDb8rapE7v52", Amount::new(1372917)),
	("5ECqBsLWy5XdRGj4fSJ6gCBPd6ez9FtkM3TDGkQ2qZQg9bM6", Amount::new(1372917)),
	("5vzBV3mvhnDLo2TAYCj4pvP8SfyqL69E5V2e4kU8ghw5JB4F", Amount::new(26666667)),
	("5uJbv6L4TnJGX6FQ59H4C9Z7hXg8xzcX7rYURqYXCvW2w6uy", Amount::new(37500000)),
	("5vMajFzUZMQdTBgPen7DKYVWy4HjzVDeAJdrWSRBjy5i2cFX", Amount::new(50000000)),
	("5yFTVTjVvtnHjNrLYkuGBzDc76qN6U27FjQgThRC9bu1HZCH", Amount::new(333333)),
	("5xUUtcenugieijDo1VvnoF3Ms7EazvrZzjfZyXG5nRVj5jrJ", Amount::new(6250000)),
	("16A5r6WjR2DxE5iezedrEEf4aSrKkNMwF1Py8ir52pk1Z5oN", Amount::new(13333333)),
	("5utPhSsz5E3QrxifY4ZuFogoqEm5Q8SoGSHuitd448rSGtvU", Amount::new(1_666_667)),
	("5F1KmMJx937D4oh2n1d4eDDRqTzJAsDFU2mm2nCFbAJcnXpk", Amount::new(2666667)),
	("5y4qrzALpE8AzJ76ShUq7QvsfxqfVPW8AmjABSiPMDAgjcco", Amount::new(52501400)),
	("5yTcSpSi3FtfiA7pCMhVGYx7hoEJNwK5N3mdBNeFtaaGkGz1", Amount::new(52501400)),
	("13M4vA18KW4UjEaeUnNoNdhLyeAukeet9H2fpzV3AdGvJVB3", Amount::new(26000000)),
	("5uz9LhJ6C31PiZKXzy9Fs1hqKH5s4mVVyQSYsUb6s6Yss7NB", Amount::new(1333333)),
	("E9jws3FbBufHQw2P2Eo7TiU4bNzjGPXGHAvAyfTXuixsobo", Amount::new(12500000)),
	("E5VMenvTm9cJABoAxeAJ7VYNx7j43x85GFynoRR7KP2gEDS", Amount::new(166667)),
	("Dfk5jfmjkXJtK4RWqDaQj2gdw2L1ibrqbaDsaDfYa5vZsD9", Amount::new(4000000)),
	("16aLkyVcgi6m5tq4s4gchT7qVzv3qH5PFVRmNGg6zx2r6iaD", Amount::new(2000000)),
	("16K4snMkvjg4Y1gKAhNSZxt3FyDar9tek6QaF7gHRV3tKwmR", Amount::new(500000)),
	("15VLhQceqa6JzLU28RSBXHsoJoCCyrVArTh9YBvn4ZRxcy8Q", Amount::new(1250000)),
	("12Wc7hoP5JZ8wLdcPp8jPHA3S2RCVpwfCQwBg9JSVKELuTVH", Amount::new(2500000)),
	("1aDGcEoUorbPjLyuMPpgvLAsqXHj3UKcS9jDkcqgMkHmEy1", Amount::new(1250000)),
	("14uBVMcShyvt1rBrrHmfMFyNeS9ftiuoEtVi25fjjChiYN4K", Amount::new(1250000)),
	("14ocFVyrU2VQic2exFLFJiRpuTKJZHb8vHYfkRA1KUuuMLfD", Amount::new(1250000)),
	("156Ponz56YTrQhWj3CkZ2ytJPSBiiYUW2Lc7MVvSHHTz85nV", Amount::new(1250000)),
	("14So37oczpZCDsebC3EmamfNRtejdGL1ZxqELqM2NxKpPkw3", Amount::new(1250000)),
	("5HQSKEsdjU4S5kM5tHyxcuSKNK5Le8TRDCKYPagmNuZhF6ZP", Amount::new(166667)),
	("5HKcQUswFRvRzRyfxrXrvLmRQ99vb7sjJkkJ94X1r4Ccku4B", Amount::new(33333333)),
	("5yyASSiHJCr4DvB5a9XzRPi8Wx9smBF5qtW9441SiFQasYKB", Amount::new(66666667)),
	("Ehbzk43EGrPFCfiiDKNSoYME3pYfWotYZY8bRawgTMG5vUi", Amount::new(13333333)),
	("5G8sYDcPHNFbWo3QkW31y8J5kzdbefnZp6pRZEDjaPsDPhh2", Amount::new(3333333)),
	("5GgjZECB6XsH3iao7rg6dDbMG9urjsWVjinDBF2ngqWFxyoC", Amount::new(1333333)),
	("5y9TtLhzzYg4RoQfRxovKRU5jUyD2ckSQ5iRqjuFwo6xbQFy", Amount::new(12500000)),
	("5zEmyotMQSzczb4scMPukPvRt7s3V7HFb2z9s8emTV1drpU1", Amount::new(16_666_667)),
	("5GBjnZwBiuAQKnR5nybF8RdJkrobDzaHBuwEyuFABvS3tDR6", Amount::new(16666667)),
	("DbJbiPedCn22Bk4AFPneoudGEoQ9bN3DctMaBsJD3JXt8pK", Amount::new(1875000)),
	("5wfGoM2XDBCKXnbZQHsZHzhjKLvdQM23TcT421YJ7K8sCxFh", Amount::new(12500000)),
	("5waasw3YdNBZXCcWViL1NRi8LiZq65vZ3VRKZFHjowfKjaxZ", Amount::new(5000000)),
	("HfQ2voxoqYBMFYW1KReRiVS7PRKKoViyESBRdnkBCuzQyqs", Amount::new(8333333)),
	("168MYZighTqrCNFqdu6BdiXvESsTXSsN8UuTrUGqJMgkuJrB", Amount::new(7333333)),
	("15JZBR1PBubuDG3fjnE5oWQ44fP5yXnnkhTcAb3PvqDM24XD", Amount::new(5000000)),
];

const NATIVE_COUNCIL: &[&str] = &[
	"5xyAEAzbbAx5rD65AUyH4BqsQTLCUMt2LUm5kLNfkRETSbD4",
	"5FCbdwsZVG6cttwW37daEHJoPPqY4Jj3givgFhc9UNqGGhpc",
	"5z6Uxn8y1GMFuqHh585fzm57LsBUup2469RKsf3wVoiJCzH1",
	"5vkv4jjX29yfYBTnJEFmBukTZC4myfKgfgNe64dvX4aTwX4n",
	"5yASMqX6bH8qx6Ed66HWLNKtAGaKeD2Z4eUib6927LaSJBZZ",
	"5wtELdwzqyuWCR78W1jDyQ8nb1jViGcgVZn4BqQbGYU7gCEi",
	"5CJUn2syLhz6nJVdLpxNSxkoBYyGHWurE8UxYKZ5obx4FXH4",
	"5x27jpexjzR1G5cM4aSb9iZH7CskQ4n2ocVkR3o16Wkarsbu",
	"5vFtm6WHwPGsCSunvmxN6FqReV1bcWw75FD7KUiZvV7vsy8t",
	"5x79GYLQNxcmryb8jRQkxkWs9LSTzKwPn32NiQd6WXsuDrb4",
	"5v8VpcLqouh25Q6cLM4CTvHfYT5XqNNiQBGVtcS768Cd59RX",
];

const TECH_COUNCIL: &[&str] = &[
	"5GQjasGFwLDnyTcBvvUwRA1smNU8JN15MPYx2TV8N62iopw2",
	"5GF8HUxZ1tCHWVZMoGVqDWe8EtdnKznHEbaguRQNqvMkw34H",
	"5uVVWApbzcdgrkfYDRSn42Zcs3GnT9ypU7EHnzCuEsLyztm3",
	"5FCCuxH6VuMdCdsqiYAHg515FJopNvSYVdQCHGBVunAdKh3M",
	"5HY3FuywmNdPfVkG53ZkSruJcLLxJN51Skei8rqYWFFhAu2r",
	"5wXghvLSiSksRytoyGPq9mQ12CsfcP49tng3XSMJYEYgcWZr",
	"5EyK95UCZaWbD3YUvvQKgaSYeP9L6rhd6ddi25xLt3crPCBX",
];

const TREASURY_MSIG: &[&str] = &[
	"5uuVmG72PzKfYNewVUfVmVxQMRXhjhB6cJnzLhqNCxVu3dGp",
	"5C5RYpMfwqbJTbTCtsG3MUsuD7782Xv2LhohjVBGzG5iucuL",
	"5urB7uAgHFdgFRuohhQNHMdGHP4uRrjaKVnRUyMk3RP9bJpt",
	"5uQ6tWbxJvoS84QE8XrM9QApiwsxXYmjfXm28DaoAiDCj6eF",
	"5uJueaCukS1qFXPTeXycCBd3SVbumV38i3mpbG8gPkAAip7r",
	"5v2R5BCdBfyWJTikPB1zdcBB2W73Ut4DVPTvsraAkqsL9bkS",
	"5CqZ9mELKwk7hV6WKVzm3M5GnXwaibFCh714WeW54aFey88E",
	"5u7x85XYQ22Yxj6XZHcr6kQk2xhd2i7fNECAVRQpGmbNK8dJ",
	"5x29WEJkvixjzPqAwXMSYatyZ1DpjHcT6DYLJkTyXwmENoks",
	"5weoaaztdvn88s6Cn23zahxe8VCfQ3UsKJeA57LM4j5WJdjX",
	"5z95uz5LfGHFSukyEBFezeebd278UmLvKovi8hfoJxcwRdaw",
];

const ECOSYSTEM_INCENTIVES_MSIG: &[&str] = &[
	"5ykL57L8KLrUn5mGKg23WHtQAuBada2E5rtuoNv1NoTSBnP2",
	"5HN4nzgzCWqT5oqmycpADH8efNxJsywYuTJExbyPrcSrfLhu",
	"5xFwuUQsMnkG92xgcLQ2XyvGpZjq7bGTevSqZCbMAD7yHUjF",
	"5wWsrkiWhmMiy7DzjurYKSQ9zbEohvZPLEqhHVuqpwAYrBXZ",
	"5uQjawk289JaAjK3cQJaBzQ31hdSdSNQAUD1qVc2UkzEyAVE",
	"5vdHj3dgwAeUUSyg89MjxU3ksuPNthhkENDVYtQAbQLRMetX",
	"5HQaiGrnau6NqvXFikzZ9pz9TpceY4gXiyprZCYVtvbMwhBo",
	"5zEQ4ourpwXdtn1s5Pm3hEZcfE9wqcQXV1WM2DvajLcRZaUu",
	"5z4feDAoLtsBTcF9a72uauYtkdU6y3WNYLtBN8det3kdeX8b",
	"5wCX2QU7wTd4wMHysQF6LQLJ2yynrgvmEpsUAJeYTt3KkVjM",
	"5vhwKyotaohcRQUp9Lr8wUgUBwfaDqZZYSXu31sQQ3n7Yvaf",
];

const LIQUIDITY_PROGRAMS_MSIG: &[&str] = &[
	"5yPPBRbLdyGZX79pP2iHFdgzjEjmCY29Tm8qB6PebyWGdpSX",
	"5FsBvfR6A1otoZMGYHRGJ4UHPo1nRrNuFhrBZ84FbKK8o8mU",
	"5wwB1mdc7HTuedtcSrZ8yvP5715Nb7FKdojHMr776gGc5ipQ",
	"5vm73wPXjREjoSVA3ycfNLbjMPJkFLfc7hHjV7AqXzqKLaX8",
	"5uV7onK9rBbo17yQjy833URFMtAri6N8ieb3eocyGZjRLmS9",
	"5xpsDMcor2rmkDVPfzJ3uDUBjEo7Wf5C38Kv3Cje3iTfq9ZX",
	"5ENfibEkoTnkPnmWCf3KpWTkgMh9JEieVkTiwSRxL4bEbQmr",
	"5zEBXadXr7WEgo32rSxkzkzN4VKvqqZs8Xmv9aM8CMsPns9F",
	"5wA6tsLsdoZ4jxkTXWSj6zsCzNqutMDQ9RnhkqgG6p41kAcf",
	"5z7XXgsGjkmpN3Yd8kazWkTFmFFd7yPMF8M7qynvNT1YE8FC",
	"5xnXawcszffzPjFtEWiCPsgqVKRfdkZa2vciePhjpEUwUkdT",
];

const TEAM_MSIG: &[&str] = &[
	"5wFGWTb9pxFXbFjKyST2JSmVLz7dHVy6bKdxEGh3Y5iCa5v8",
	"5uiQYPKrG72RRikpvJfuQE8ghKB47rKxnEuiymCywzsKRUMb",
	"5yX8dWEBdWivoPh7tkxoby2wkD78sgbm3Bk2Rx4jsGugwXdN",
	"5vJwkhJCnpjq2R9isLnnSAXn3HBVVWRkvG5SabM5XaUGYrb1",
	"5E9sbF2Ty4eq6oE69kX7EVBNc4CFZMzYn7B9gyyHc9natSEu",
];

const OPERATIONAL_TOTAL_ACCOUNTS: usize = INVESTORS.len() +
	NATIVE_COUNCIL.len() +
	TECH_COUNCIL.len() +
	TREASURY_MSIG.len() +
	ECOSYSTEM_INCENTIVES_MSIG.len() +
	LIQUIDITY_PROGRAMS_MSIG.len() +
	TEAM_MSIG.len();

const OPERATIONAL_DUST: Amount<Raw> = Amount::new(5);

const TREASURY_MSIG_THRESHOLD: u16 = 8;
const TREASURY_MSIG_FUNDS: Amount<Raw> = Amount::new(1_800_297_201);

const ECOSYSTEM_INCENTIVES_MSIG_THRESHOLD: u16 = 8;
const ECOSYSTEM_INCENTIVES_MSIG_FUNDS: Amount<Raw> = Amount::new(1_000_000_000);

const LIQUIDITY_PROGRAMS_MSIG_THRESHOLD: u16 = 8;
const LIQUIDITY_PROGRAMS_MSIG_FUNDS: Amount<Raw> = Amount::new(1_500_000_000);

const TEAM_MSIG_THRESHOLD: u16 = 3;
const TEAM_MSIG_FUNDS: Amount<Raw> = Amount::new(2_000_000_000);

const INVESTORS_VESTING_START_YEAR: u32 = 2023;
const INVESTORS_VESTING_START_MONTH: u32 = 04;
const INVESTORS_VESTING_START_DAY: u32 = 01;

const INVESTORS_VESTING_END_YEAR: u32 = 2025;
const INVESTORS_VESTING_END_MONTH: u32 = 01;
const INVESTORS_VESTING_END_DAY: u32 = 01;

const INFRA_PROVIDERS_VESTING_START_YEAR: u32 = 2023;
const INFRA_PROVIDERS_VESTING_START_MONTH: u32 = 04;
const INFRA_PROVIDERS_VESTING_START_DAY: u32 = 01;

const INFRA_PROVIDERS_VESTING_END_YEAR: u32 = 2023;
const INFRA_PROVIDERS_VESTING_END_MONTH: u32 = 07;
const INFRA_PROVIDERS_VESTING_END_DAY: u32 = 01;

const VESTING_CLAIM_STEP_DAYS: u32 = 1;

const ORACLE_FUNDS: Amount<Raw> = Amount::new(72_132_699);

const fn _infrastructure_total_funds() -> Amount<Raw> {
	let mut amount = Amount::new(0);
	let mut i = 0;
	while i < INFRASTRUCTURE_PROVIDERS.len() {
		let (_, reward) = INFRASTRUCTURE_PROVIDERS[i];
		amount = amount + reward;
		i += 1;
	}
	amount
}

// The value was rounded down, 700 -> 699
const _INFRASTRUCTURE_FUNDS_TOTAL_EXPECTED: Amount<Raw> = Amount::new(72_132_699);
const _INFRASTRUCTURE_FUNDS_ASSERT: () =
	assert!(_infrastructure_total_funds() == _INFRASTRUCTURE_FUNDS_TOTAL_EXPECTED);

const fn _investor_total_funds() -> Amount<Raw> {
	let mut amount = Amount::new(0);
	let mut i = 0;
	while i < INVESTORS.len() {
		let (_, reward) = INVESTORS[i];
		amount = amount + reward;
		i += 1;
	}
	amount
}

const _OPERATIONAL_FUNDS_TOTAL: Amount<Raw> = _investor_total_funds() +
	TREASURY_MSIG_FUNDS +
	ECOSYSTEM_INCENTIVES_MSIG_FUNDS +
	LIQUIDITY_PROGRAMS_MSIG_FUNDS +
	TEAM_MSIG_FUNDS;

// We don't compute the crowdloan here.
const _OPERATIONAL_FUNDS_TOTAL_EXPECTED: Amount<Raw> = Amount::new(7_000_000_000);
const _OPERATIONAL_FUNDS_ASSERT: () =
	assert!(_OPERATIONAL_FUNDS_TOTAL == _OPERATIONAL_FUNDS_TOTAL_EXPECTED);

#[derive(Debug)]
pub enum OperationalError {
	Common(CommonError),
	ApiFailure(substrate_api_client::ApiClientError),
}

impl From<substrate_api_client::ApiClientError> for OperationalError {
	fn from(x: substrate_api_client::ApiClientError) -> Self {
		Self::ApiFailure(x)
	}
}

impl From<CommonError> for OperationalError {
	fn from(x: CommonError) -> Self {
		Self::Common(x)
	}
}

pub fn setup_native_council(
	api: Api<sp_core::sr25519::Pair, WsRpcClient, AssetTipExtrinsicParams>,
) -> Result<(), OperationalError> {
	let native_council_members = NATIVE_COUNCIL
		.iter()
		.map(|x| extract_account(x))
		.collect::<Result<Vec<_>, _>>()?;

	log::info!("Submitting native council...");
	let tx_hash = api_wrap::<_, OperationalError>(
		api.send_extrinsic(
			compose_extrinsic!(
				api,
				"Sudo",
				"sudo",
				compose_call!(
					api.metadata.clone(),
					"Council",
					"set_members",
					// NewMembers
					native_council_members,
					// Prime
					None::<AccountId32>,
					// OldCount
					0u32
				)
			)
			.hex_encode(),
			XtStatus::InBlock,
		),
	)?;
	log::info!("Native council submitted, hash={:?}", tx_hash);

	Ok(())
}

pub fn setup_technical_council(
	api: Api<sp_core::sr25519::Pair, WsRpcClient, AssetTipExtrinsicParams>,
) -> Result<(), OperationalError> {
	let technical_collective_members =
		TECH_COUNCIL.iter().map(|x| extract_account(x)).collect::<Result<Vec<_>, _>>()?;

	log::info!("Submitting native council update...");
	let tx_hash = api_wrap::<_, OperationalError>(
		api.send_extrinsic(
			compose_extrinsic!(
				api,
				"Sudo",
				"sudo",
				compose_call!(
					api.metadata.clone(),
					"TechnicalCollective",
					"set_members",
					// NewMembers
					technical_collective_members,
					// Prime
					None::<AccountId32>,
					// OldCount
					0u32
				)
			)
			.hex_encode(),
			XtStatus::InBlock,
		),
	)?;
	log::info!("Native council update submitted, hash={:?}", tx_hash);

	Ok(())
}

pub fn fund_dust(
	api: Api<sp_core::sr25519::Pair, WsRpcClient, AssetTipExtrinsicParams>,
) -> Result<(), OperationalError> {
	let treasury_msig = multi_account_id(
		Ss58AddressFormatRegistry::PicassoAccount,
		TREASURY_MSIG,
		TREASURY_MSIG_THRESHOLD,
	)?;

	let fund_from_treasury = |account, amount: Amount<Raw>| {
		compose_call!(
			api.metadata,
			"Sudo",
			"sudo",
			compose_call!(
				api.metadata,
				"Assets",
				"force_transfer_native",
				GenericAddress::Id(treasury_msig.clone()),
				GenericAddress::Id(account),
				Compact(u128::from(Amount::<Canonical>::from(amount))),
				false
			)
		)
	};

	let investors_account =
		INVESTORS.iter().cloned().map(|(account, _)| account).collect::<Vec<_>>();

	let operational_accounts = [
		&investors_account,
		NATIVE_COUNCIL,
		TECH_COUNCIL,
		TREASURY_MSIG,
		ECOSYSTEM_INCENTIVES_MSIG,
		LIQUIDITY_PROGRAMS_MSIG,
		TEAM_MSIG,
	]
	.iter()
	.cloned()
	.flatten()
	.collect::<Vec<_>>();

	assert_eq!(operational_accounts.iter().len(), OPERATIONAL_TOTAL_ACCOUNTS);

	// Ensure unique accounts get dusted
	let unique_operational_accounts = operational_accounts.iter().collect::<HashSet<_>>();

	let normalized_accounts = unique_operational_accounts
		.into_iter()
		.map(|account_ss58| extract_account(account_ss58))
		.collect::<Result<Vec<_>, _>>()?;

	let batch = normalized_accounts
		.into_iter()
		.map(|account| fund_from_treasury(account, OPERATIONAL_DUST))
		.collect::<Vec<_>>();

	log::info!("Submitting batch mint...");
	let tx_hash = api_wrap::<_, OperationalError>(api.send_extrinsic(
		compose_extrinsic!(api, "Utility", "batch_all", Batch { calls: batch }).hex_encode(),
		XtStatus::InBlock,
	))?;
	log::info!("Batch mint submitted, hash={:?}", tx_hash);

	Ok(())
}

pub fn fund_investors(
	api: Api<sp_core::sr25519::Pair, WsRpcClient, AssetTipExtrinsicParams>,
) -> Result<(), OperationalError> {
	let team_msig = multi_account_id(
		Ss58AddressFormatRegistry::PicassoAccount,
		TEAM_MSIG,
		TEAM_MSIG_THRESHOLD,
	)?;

	let total_funds = _investor_total_funds();

	log::info!("Minting investors amount on treasury...");
	let tx_hash = api_wrap::<_, OperationalError>(
		api.send_extrinsic(
			compose_extrinsic!(
				api,
				"Sudo",
				"sudo",
				compose_call!(
					api.metadata,
					"Assets",
					"mint_into",
					CurrencyId::PICA,
					GenericAddress::Id(team_msig.clone()),
					Compact(u128::from(Amount::<Canonical>::from(total_funds)))
				)
			)
			.hex_encode(),
			XtStatus::InBlock,
		),
	)?;
	log::info!("Minted investors funds, tx={:?}", tx_hash);

	let start_date = NaiveDate::from_ymd(
		INVESTORS_VESTING_START_YEAR as _,
		INVESTORS_VESTING_START_MONTH,
		INVESTORS_VESTING_START_DAY,
	)
	.and_hms(0, 0, 0);
	let end_date = NaiveDate::from_ymd(
		INVESTORS_VESTING_END_YEAR as _,
		INVESTORS_VESTING_END_MONTH,
		INVESTORS_VESTING_END_DAY,
	)
	.and_hms(0, 0, 0);
	let number_of_days = (end_date - start_date).num_days();

	let batch_vesting = INVESTORS
		.iter()
		.map(|(investor_account, reward)| {
			let account = extract_account(investor_account)?;
			let schedule = VestingScheduleInfo::<u32, u64, u128> {
				window: VestingWindow::MomentBased {
					start: start_date.timestamp() as u64,
					period: Duration::days(VESTING_CLAIM_STEP_DAYS as _).num_seconds() as u64,
				},
				period_count: number_of_days as u32,
				per_period: u128::from(Amount::<Canonical>::from(*reward)) /
					(number_of_days as u128),
			};
			Ok(compose_call!(
				api.metadata,
				"Sudo",
				"sudo",
				compose_call!(
					api.metadata,
					"Vesting",
					"vested_transfer",
					GenericAddress::Id(team_msig.clone()),
					GenericAddress::Id(account),
					CurrencyId::PICA,
					schedule
				)
			))
		})
		.collect::<Result<Vec<_>, OperationalError>>()?;

	log::info!("Submitting batch vesting...");
	let tx_hash = api_wrap::<_, OperationalError>(
		api.send_extrinsic(
			compose_extrinsic!(
				api,
				"Utility",
				"batch_all",
				Batch { calls: batch_vesting.to_vec() }
			)
			.hex_encode(),
			XtStatus::InBlock,
		),
	)?;
	log::info!("Batch vesting submitted, hash={:?}", tx_hash);

	Ok(())
}

pub fn fund_multisigs(
	api: Api<sp_core::sr25519::Pair, WsRpcClient, AssetTipExtrinsicParams>,
) -> Result<(), OperationalError> {
	let fund_account = |account, currency: CurrencyId, amount: Amount<Raw>| {
		compose_call!(
			api.metadata,
			"Sudo",
			"sudo",
			compose_call!(
				api.metadata,
				"Assets",
				"mint_into",
				currency,
				GenericAddress::Id(account),
				Compact(u128::from(Amount::<Canonical>::from(amount)))
			)
		)
	};

	let treasury_msig = multi_account_id(
		Ss58AddressFormatRegistry::PicassoAccount,
		TREASURY_MSIG,
		TREASURY_MSIG_THRESHOLD,
	)?;
	log::info!(
		"Treasury MSIG: {}",
		treasury_msig.to_ss58check_with_version(Ss58AddressFormatRegistry::PicassoAccount.into())
	);

	let ecosystem_incentives_msig = multi_account_id(
		Ss58AddressFormatRegistry::PicassoAccount,
		ECOSYSTEM_INCENTIVES_MSIG,
		ECOSYSTEM_INCENTIVES_MSIG_THRESHOLD,
	)?;
	log::info!(
		"Ecosystem Incentives MSIG: {}",
		ecosystem_incentives_msig
			.to_ss58check_with_version(Ss58AddressFormatRegistry::PicassoAccount.into())
	);

	let liquidity_programs_msig = multi_account_id(
		Ss58AddressFormatRegistry::PicassoAccount,
		LIQUIDITY_PROGRAMS_MSIG,
		LIQUIDITY_PROGRAMS_MSIG_THRESHOLD,
	)?;
	log::info!(
		"Liquidity Programs MSIG: {}",
		liquidity_programs_msig
			.to_ss58check_with_version(Ss58AddressFormatRegistry::PicassoAccount.into())
	);

	let team_msig = multi_account_id(
		Ss58AddressFormatRegistry::PicassoAccount,
		TEAM_MSIG,
		TEAM_MSIG_THRESHOLD,
	)?;
	log::info!(
		"Team MSIG: {}",
		team_msig.to_ss58check_with_version(Ss58AddressFormatRegistry::PicassoAccount.into())
	);

	log::info!("Funding Team msig with {:?} PICA.", TEAM_MSIG_FUNDS);
	log::info!("Funding Treasury msig with {:?} PICA.", TREASURY_MSIG_FUNDS);
	log::info!(
		"Funding Ecosystem Incentives msig with {:?} PICA.",
		ECOSYSTEM_INCENTIVES_MSIG_FUNDS
	);
	log::info!("Funding Liquidity Programs msig with {:?} PICA.", LIQUIDITY_PROGRAMS_MSIG_FUNDS);

	let batch_msig = [
		fund_account(team_msig, CurrencyId::PICA, TEAM_MSIG_FUNDS),
		fund_account(treasury_msig, CurrencyId::PICA, TREASURY_MSIG_FUNDS),
		fund_account(ecosystem_incentives_msig, CurrencyId::PICA, ECOSYSTEM_INCENTIVES_MSIG_FUNDS),
		fund_account(liquidity_programs_msig, CurrencyId::PICA, LIQUIDITY_PROGRAMS_MSIG_FUNDS),
	];

	log::info!("Submitting batch mint...");
	let tx_hash = api_wrap::<_, OperationalError>(
		api.send_extrinsic(
			compose_extrinsic!(api, "Utility", "batch_all", Batch { calls: batch_msig.to_vec() })
				.hex_encode(),
			XtStatus::InBlock,
		),
	)?;
	log::info!("Batch mint submitted, hash={:?}", tx_hash);

	Ok(())
}
