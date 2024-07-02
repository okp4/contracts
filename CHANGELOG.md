# ØKP4 contracts

## [5.0.0](https://github.com/axone-protocol/contracts/compare/v4.1.0...v5.0.0) (2024-04-02)


### ⚠ BREAKING CHANGES

* **law-stone:** return a prolog error when law is broken

### Features

* **dataverse:** support claim with named node hierarchy ([2081625](https://github.com/axone-protocol/contracts/commit/20816255966bad0dd943d788df75784731eba8d1))


### Bug Fixes

* **dataverse:** avoid possible conflicts between nodes ([73c65e7](https://github.com/axone-protocol/contracts/commit/73c65e762a82948af1cb23f7c6833d1dc57edcac))
* **dataverse:** prevent vc to contain reserved predicates ([2917fdb](https://github.com/axone-protocol/contracts/commit/2917fdb6f72dfeae4ca2814b72e5ac6acba1605d))


### Performance Improvements

* **rdf:** make id issuer return references ([64a227f](https://github.com/axone-protocol/contracts/commit/64a227fd76d65b97f35374a7b92632bb571b6932))


### Code Refactoring

* **law-stone:** return a prolog error when law is broken ([41f9de5](https://github.com/axone-protocol/contracts/commit/41f9de556f4a49c8d1fbc1bdfd73e7d71cd4492e))

## [4.1.0](https://github.com/axone-protocol/contracts/compare/v4.0.0...v4.1.0) (2024-03-17)


### Features

* **dataverse:** implements dataverse query ([f93641d](https://github.com/axone-protocol/contracts/commit/f93641dde61edb58ce26fca474e5df421f563a94))

## [4.0.0](https://github.com/axone-protocol/contracts/compare/v3.0.0...v4.0.0) (2024-03-05)


### ⚠ BREAKING CHANGES

* **law-stone:** match v7 logic module model
* **logic-bindings:** match v7okp4d logic module model
* **cognitarium:** rework construct clause based on template
* **cognitarium:** use var or named node as msg pattern predicate
* **cognitarium:** rework delete input to remove template bnodes
* **law-stone:** update needed by new logic bindings

### Features

* **cognitarium-client:** add basic client ([d4ab756](https://github.com/axone-protocol/contracts/commit/d4ab756b1d07b5c5175d86e7b792668f33cb2e89))
* **cognitarium:** add select mapping to msg on describe ([a2010a7](https://github.com/axone-protocol/contracts/commit/a2010a72c14c906c5c528b4566d58bb6827a32fa))
* **cognitarium:** add select msg mapping layer ([ef789f8](https://github.com/axone-protocol/contracts/commit/ef789f8f6a73d9277605934a6ec3a15936d6054c))
* **cognitarium:** allow delete without where clause ([e22d9b9](https://github.com/axone-protocol/contracts/commit/e22d9b93a501f9feae7a2c0788f14f17a31641ba))
* **cognitarium:** generate bnode identifiers on query ([93acb93](https://github.com/axone-protocol/contracts/commit/93acb9392032f10c273d5b02433251ec774be86d))
* **cognitarium:** implement construct query ([7a5490e](https://github.com/axone-protocol/contracts/commit/7a5490efb40949256d337cfd4e5600c2b7ffba21))
* **cognitarium:** introduce blank node id counter state ([2e60a12](https://github.com/axone-protocol/contracts/commit/2e60a1203095fa7749e317b4651437f3d3fe5ff6))
* **cognitarium:** issue new id for bnode provided in construct ([f22eb81](https://github.com/axone-protocol/contracts/commit/f22eb817d4f4e047e69b41a68b085a440ca43ba7))
* **cognitarium:** make construct rely on atoms iterator ([6ebbf83](https://github.com/axone-protocol/contracts/commit/6ebbf837d1bfd2a744bb8044934a07615dc6ae21))
* **cognitarium:** make delete msg rely on triple iterator ([1fdc691](https://github.com/axone-protocol/contracts/commit/1fdc6918e378f68ff46f230674f97d98888d5710))
* **cognitarium:** make describe msg rely on atom iterator ([78648b4](https://github.com/axone-protocol/contracts/commit/78648b49688d5c4bfa6a8b2d5777fe2dcd4b923c))
* **cognitarium:** offer additional querier iterators ([292ef7d](https://github.com/axone-protocol/contracts/commit/292ef7df229c37c060cfee85cac7bd0e77988a30))
* **cognitarium:** reuse where clause in construct empty ([61ff078](https://github.com/axone-protocol/contracts/commit/61ff0782de781381157327f3b83a6b0d4c7fd715))
* **cognitarium:** rework construct clause based on template ([f15d822](https://github.com/axone-protocol/contracts/commit/f15d8228c8a92efcb8532152e15eaffabe71fe23))
* **cognitarium:** rework delete input to remove template bnodes ([7dfb48e](https://github.com/axone-protocol/contracts/commit/7dfb48ebf8c2ae1fa69616f9de957d3576889fb7))
* **cognitarium:** update triple state model with u128 as bnode ([1a492e7](https://github.com/axone-protocol/contracts/commit/1a492e7463c0e34288a2cbefded5488f3ca380f3))
* **cognitarium:** use only state model in querier ([3e6d8af](https://github.com/axone-protocol/contracts/commit/3e6d8afdee2a3ec682e7030c8f458974527f7b81))
* **dataverse:** add basic internal state ([1c05850](https://github.com/axone-protocol/contracts/commit/1c05850dd8c1f87b8680ce19c7e8c24b1a497999))
* **dataverse:** add DataIntegrityProof vc support ([a0d055d](https://github.com/axone-protocol/contracts/commit/a0d055d4181997d54403c759b33216786958bcb9))
* **dataverse:** add dedicated errors to credentials ([4e8890e](https://github.com/axone-protocol/contracts/commit/4e8890e77e4a65a8b9b47b50c7472f7ea4287fea))
* **dataverse:** add ecdsa secp 2019 vc proof support ([46cd40c](https://github.com/axone-protocol/contracts/commit/46cd40c383f72ff27dbdc4a8dec768aa04cd7b9c))
* **dataverse:** add secp256k1 crypto support ([47462be](https://github.com/axone-protocol/contracts/commit/47462bed1b977fe521c76ff97a6f92bf27e1a866))
* **dataverse:** add support of Ed25519Signature2018 proof ([ede8320](https://github.com/axone-protocol/contracts/commit/ede83205d68c9e5aa912e1c868f88f4711df3f8b))
* **dataverse:** add support to parse ed25519 2020 proofs ([c6c5540](https://github.com/axone-protocol/contracts/commit/c6c554022d928e130ed10ab63f396196f8fa4237))
* **dataverse:** add triple store config to instantiate msg ([0a129c5](https://github.com/axone-protocol/contracts/commit/0a129c587fd9a1d5957f282c7e3352f5f751b26b))
* **dataverse:** allow no proof in vc parsing according to spec ([6c56cd0](https://github.com/axone-protocol/contracts/commit/6c56cd096478f17cee80592c3e640aaf8ac87293))
* **dataverse:** dissociate triple store msg limits struct ([c142976](https://github.com/axone-protocol/contracts/commit/c142976a035348f1bc32758c316d1b3848a835d7))
* **dataverse:** garantee claim storage format ([b2f6d43](https://github.com/axone-protocol/contracts/commit/b2f6d43ce3b32e4a917987e59622c1073f80ec0b))
* **dataverse:** impl rdf serialization of dataverse cred ([d4da31d](https://github.com/axone-protocol/contracts/commit/d4da31d4945b3c303a620d7a42b8b65e0958deb8))
* **dataverse:** implement instantiate msg ([f5fade2](https://github.com/axone-protocol/contracts/commit/f5fade26c110a71c82831207f5e38a618f76b457))
* **dataverse:** implement vc proof parsing ([bd826f9](https://github.com/axone-protocol/contracts/commit/bd826f924f8884636c0e1084251fea8f5084f374))
* **dataverse:** implements claims registering in cognitarium ([385426e](https://github.com/axone-protocol/contracts/commit/385426ee413750bd8f414c761f432d3ae784c78d))
* **dataverse:** implements dummy submitclaims exec msg ([948331f](https://github.com/axone-protocol/contracts/commit/948331f8957eb6892dbd051789a80a7c8b2320cc))
* **dataverse:** implements proof verification ([ae8f97e](https://github.com/axone-protocol/contracts/commit/ae8f97e3242a6ffbf68ece68486ba93e329a2cd4))
* **dataverse:** implements VC rdf parsing ([a972284](https://github.com/axone-protocol/contracts/commit/a97228457c616bdd518155ec2f04aa914607d3d3))
* **dataverse:** initialize blueprint smart contract ([fde585b](https://github.com/axone-protocol/contracts/commit/fde585b6ab05c12324d492ebd9a55a6ae3bd5240))
* **dataverse:** introduce dataverse credential model ([3347455](https://github.com/axone-protocol/contracts/commit/33474559727da5cc5991d732ddf87bbc00ca20ce))
* **dataverse:** introduce subject ids for RDF resource designation ([bc37b48](https://github.com/axone-protocol/contracts/commit/bc37b48385db812283deb74d8a3a23759f4231b2))
* **dataverse:** properly persists claims in related exec msg ([6abb559](https://github.com/axone-protocol/contracts/commit/6abb559566a24942f9d888f94c6fdf39b2388af7))
* **dataverse:** rework SubmitClaims msg doc ([1489f95](https://github.com/axone-protocol/contracts/commit/1489f95e8305c5fed943c69fe54353b16c5e6995))
* **dataverse:** specify AttachMetadata execute message ([99684f5](https://github.com/axone-protocol/contracts/commit/99684f5404df1b8d86b2d70f2786cb5c71555c21))
* **dataverse:** specify contract instanciation ([9168c68](https://github.com/axone-protocol/contracts/commit/9168c68c928fabb0504eb30ca2acbeaf2e1d02b2))
* **dataverse:** specify DetachMetadata execute message ([7666455](https://github.com/axone-protocol/contracts/commit/76664553df746ad706f272a809ac043816fa5431))
* **dataverse:** specify FoundZone execute message ([2c38b6b](https://github.com/axone-protocol/contracts/commit/2c38b6b8cd5cc1f37aca5e840d99ba8d7a7228d5))
* **dataverse:** specify RegisterDataset execute message ([7a2f935](https://github.com/axone-protocol/contracts/commit/7a2f935bd7ee6c032b67246a779600d4f5ba0f4b))
* **dataverse:** specify RegisterService execute message ([8faf0c1](https://github.com/axone-protocol/contracts/commit/8faf0c1d2946c0f9640fbf5312f426556414548d))
* **dataverse:** specify ReviseMetadata execution message ([6a0a67a](https://github.com/axone-protocol/contracts/commit/6a0a67a9f693fea184af18d9bf635d03a01d5111))
* **dataverse:** use a dedicated type predicate for cred storage ([f74aa3b](https://github.com/axone-protocol/contracts/commit/f74aa3b3aecfc4c81ebd2787fe905ae8c1e02a86))
* **dataverse:** use Addr type for triple store state address ([89a55ee](https://github.com/axone-protocol/contracts/commit/89a55eeb39af3a1581d33f27fbff2c6716819ca7))
* **law-stone:** implement program_code query ([ef8255d](https://github.com/axone-protocol/contracts/commit/ef8255da755362602317fe15ec28a1fafd0a6193))
* **law-stone:** match v7 logic module model ([c25128a](https://github.com/axone-protocol/contracts/commit/c25128a2c4afff4d16dc57d8365cf8e086b235ce))
* **law-stone:** update needed by new logic bindings ([dcaef7a](https://github.com/axone-protocol/contracts/commit/dcaef7a13bef554939e6afa27aa13a726f7b503d))
* **logic-bindings:** make Answer match logic module format ([f1a809c](https://github.com/axone-protocol/contracts/commit/f1a809c5d2e0efae410a71f8da359543e439432a))
* **logic-bindings:** match v7 okp4d logic module model ([32e114e](https://github.com/axone-protocol/contracts/commit/32e114e6b6e2fdc7895125018d03d883fc2a8975))
* **rdf:** add basic error management ([58415de](https://github.com/axone-protocol/contracts/commit/58415dec1694ad13649d64f52f972c4f4450307e))
* **rdf:** add nquads parsing to owned model ([4b8a3e8](https://github.com/axone-protocol/contracts/commit/4b8a3e8b98b609fb514034a1ba81eadff47e40ab))
* **rdf:** add rdf dataset querying utils ([1c6c456](https://github.com/axone-protocol/contracts/commit/1c6c456c28d6094e459643a5eb24bed3eef85424))
* **rdf:** allow to get incremented raw id from issuer ([96286f7](https://github.com/axone-protocol/contracts/commit/96286f77f00a2c6f388e0030e87a9219dcecc7d9))
* **rdf:** implement owned quad model ([c2acec0](https://github.com/axone-protocol/contracts/commit/c2acec0e6de7261244be4a71ffc1457f9cfda16b))
* **rdf:** implements rdf canonicalization ([60df2a6](https://github.com/axone-protocol/contracts/commit/60df2a6a4c5b268226be4206b66a6bbfd0eaf169))


### Bug Fixes

* **cognitarium:** allow to reuse where as delete select ([2e763ee](https://github.com/axone-protocol/contracts/commit/2e763eee87c05ea569a80f3e872f2b0b89161242))
* **cognitarium:** avoir error on not found ns at plan building ([18fe57b](https://github.com/axone-protocol/contracts/commit/18fe57b5f277782593325b5e919a946babd20e92))
* **cognitarium:** consider blank node query as special variables ([e59616d](https://github.com/axone-protocol/contracts/commit/e59616d2db7b3fde54a6a83ce0927cbead43ab45))
* **cognitarium:** ensure equality of same var values in a triple pattern ([3c05fe8](https://github.com/axone-protocol/contracts/commit/3c05fe872bb1c26e362393aeb3d7cc11e94d359c))
* **cognitarium:** manage blank node variable as post state filters ([870d0f3](https://github.com/axone-protocol/contracts/commit/870d0f3dd355710e00fbefd2c9c8faa2620ed34a))
* **cognitarium:** prevent predicate blank node filter ([447bc72](https://github.com/axone-protocol/contracts/commit/447bc723e378d7ab53d5c9159bc633922413a4e8))
* **dataverse:** consider proof options in signed message ([4b1a058](https://github.com/axone-protocol/contracts/commit/4b1a05844a1d553c9ba9d0361682e8ce27e6efda))
* **dataverse:** correct some error messages in vc parsing ([087c933](https://github.com/axone-protocol/contracts/commit/087c93386d64b1f2c4bfd8eb3a7364f2244a880b))
* **dataverse:** make ed25519 verification wasm friendly ([da2e885](https://github.com/axone-protocol/contracts/commit/da2e8857b0df3dd70d2c8be2cf3131e92679aa8d))
* **dataverse:** persists claim sender addr as literal ([bd4a169](https://github.com/axone-protocol/contracts/commit/bd4a1697ce6f7ddd37f227430a66d6a6e3d7da6d))
* **dataverse:** properly extract claims from VC ([88141ba](https://github.com/axone-protocol/contracts/commit/88141ba85e51c365cbfa7f4704d5dc1c6118f833))
* **dataverse:** remove graph name from proof opts doc ([6c6dd60](https://github.com/axone-protocol/contracts/commit/6c6dd604e72d3c79592308faabb92fecc700b88a))
* **rdf:** allow duplicate hash in n degree path list ([7589054](https://github.com/axone-protocol/contracts/commit/7589054b8fb5d16826f4b2205255905a779bcdc1))
* **rdf:** make quad pattern iter skip work ([c14bf42](https://github.com/axone-protocol/contracts/commit/c14bf4248550f9ad8f9baac1e9083dc59b6bc5b3))


### Performance Improvements

* **cognitarium:** use resolved variables to construct atoms ([72766ff](https://github.com/axone-protocol/contracts/commit/72766ff6c29b3db0286e54f89282a1c3ca4d27e6))
* **cognitarium:** use resolved vars to contruct triples ([e389931](https://github.com/axone-protocol/contracts/commit/e389931dc5606383c5f914375105fd8999750d54))
* **rdf:** improve hash sortings ([6b6e629](https://github.com/axone-protocol/contracts/commit/6b6e6290fc67c59c07da804a1c2749ba31b11b87))


### Code Refactoring

* **cognitarium:** use var or named node as msg pattern predicate ([a194d1c](https://github.com/axone-protocol/contracts/commit/a194d1c8bc86219df5b75710fe7001707b51219c))

## [3.0.0](https://github.com/axone-protocol/contracts/compare/v2.1.0...v3.0.0) (2023-09-28)


### ⚠ BREAKING CHANGES

* **cognitarium:** make delete triples optional and enforce where clause

### Features

* **cognitarium:** implement DeleteData message ([758c172](https://github.com/axone-protocol/contracts/commit/758c172f6cb4c1cb88e5b73f6d37ee4c5b9d98dd))
* **cognitarium:** specify construction query ([111a73c](https://github.com/axone-protocol/contracts/commit/111a73c9039572ebf78b6b0d88535625c777a03c))
* **cognitarium:** support 'delete where {}' form ([3f2f9a0](https://github.com/axone-protocol/contracts/commit/3f2f9a05c9526778da0ce552fd370a6f2ada5528))
* **docs:** use fadroma-schema to generate docs ([ae792e0](https://github.com/axone-protocol/contracts/commit/ae792e02957da0cefbade4aa4e6565b1c602b860))
* **objectarium:** add lzma compression algorithm ([b3392f7](https://github.com/axone-protocol/contracts/commit/b3392f7f1dfd1f73969f765e02248dab8e1cbd4c))


### Bug Fixes

* **cognitarium:** decrease the store byte size upon deletion ([74c9461](https://github.com/axone-protocol/contracts/commit/74c9461b25f1125b6384df8a3e3a142ce31c1d68))
* **cognitarium:** do not count existing triples on insert ([b490f23](https://github.com/axone-protocol/contracts/commit/b490f23342e7e71b5271b5d26e505842ed208ea3))
* **cognitarium:** fix compilation issue ([296c61a](https://github.com/axone-protocol/contracts/commit/296c61ae600cc561aa4b0dbedcc508f09100e170))
* **cognitarium:** fix incorrect variable resolution on delete ([76abfcd](https://github.com/axone-protocol/contracts/commit/76abfcd964106ef33c2cc90c4e418ca97c6a0ac5))
* **cognitarium:** make successive store engine calls safe ([0fe529b](https://github.com/axone-protocol/contracts/commit/0fe529b92d6aa1a640c2e50f2a701c62a85a3275))
* **cognitarium:** update ns ref count on triple deletion ([8ad0144](https://github.com/axone-protocol/contracts/commit/8ad01447288c7a827fd70a6e959d3e542b8c55c3))
* **docs:** use fixed version of fadroma/schema ([bda29c7](https://github.com/axone-protocol/contracts/commit/bda29c762b43604ad3e15930803473fa541d0568))


### Code Refactoring

* **cognitarium:** make delete triples optional and enforce where clause ([5ce531b](https://github.com/axone-protocol/contracts/commit/5ce531b66dc6f14c594dfdf551b65360e5b6f700))

## [2.1.0](https://github.com/axone-protocol/contracts/compare/v2.0.0...v2.1.0) (2023-06-27)


### Features

* **cognitarium:** add expand uri utility function ([af26038](https://github.com/axone-protocol/contracts/commit/af2603843cb761da8c4ec6ff8d7bbbd7f6ee2d1c))
* **cognitarium:** add support for blank nodes in describe ([8e736f1](https://github.com/axone-protocol/contracts/commit/8e736f10b95c6540d9432c18b0346396ab28c24d))
* **cognitarium:** add support for triples serialization ([2bf805f](https://github.com/axone-protocol/contracts/commit/2bf805f1f51850f1026ca01ed3f1a1dd743dfd23))
* **cognitarium:** implement describe query ([e21edc4](https://github.com/axone-protocol/contracts/commit/e21edc40859e1632ad2a64039c71a69f14a4849b))


### Bug Fixes

* **cognitarium:** ensure different object hash on named nodes ([92656ae](https://github.com/axone-protocol/contracts/commit/92656ae4e257e331bec9ca86a29193ea39af81b2))
* **cognitarium:** fix incorrect resource type for describe ([5876c74](https://github.com/axone-protocol/contracts/commit/5876c745e03debe7d292d10e38df4ab600502b99))

## [2.0.0](https://github.com/axone-protocol/contracts/compare/v1.0.0...v2.0.0) (2023-06-13)


### ⚠ BREAKING CHANGES

* **objectarium:** update objectarium sample according to api change

### Features

* **cagnotarium:** add nquads insert support ([f963d21](https://github.com/axone-protocol/contracts/commit/f963d21b229d6d5f49e03153cbf5f2d00d764f54))
* **cognitarium:** add dummy store query impl ([e30e507](https://github.com/axone-protocol/contracts/commit/e30e507de9fcbac01af6cc94a70951239b014bff))
* **cognitarium:** add dummy triple pattern query iterator ([ac6e42d](https://github.com/axone-protocol/contracts/commit/ac6e42de007d0f75cac0c91b265e0b21572777da))
* **cognitarium:** add max_query_limit limit ([b97835b](https://github.com/axone-protocol/contracts/commit/b97835bb68d09e282bdb77e850177d25351e2e8e))
* **cognitarium:** add namespace state ([2844aac](https://github.com/axone-protocol/contracts/commit/2844aac4ad05a929624c43c3d268c4ebc1396a1b))
* **cognitarium:** add plan builder structure ([9d374eb](https://github.com/axone-protocol/contracts/commit/9d374eb14dbbc6983dcbfb3b34ca525374d1a2b3))
* **cognitarium:** add query solutions iterator ([4b5c0ac](https://github.com/axone-protocol/contracts/commit/4b5c0acceddc23031df368b3ca78b50c0146e889))
* **cognitarium:** add rdf parsing related errors ([7c25908](https://github.com/axone-protocol/contracts/commit/7c259089e9e409a683333bc7c85bd114ff2a37e3))
* **cognitarium:** add store limits related errors ([9667847](https://github.com/axone-protocol/contracts/commit/96678479dd6936542af2d9de1fce845b1e2277d6))
* **cognitarium:** add store query ([66c4579](https://github.com/axone-protocol/contracts/commit/66c4579404bdb8b269f203db329649ea0f1c48af))
* **cognitarium:** build query plan from basic graph pattern ([0ef6f1f](https://github.com/axone-protocol/contracts/commit/0ef6f1f73948c581a8479d26bf40824d733f2a03))
* **cognitarium:** build query plan from where clause ([b57e3a0](https://github.com/axone-protocol/contracts/commit/b57e3a01b024b8230ee6599fc68560c54a005701))
* **cognitarium:** define query engine structure ([6a8d7b9](https://github.com/axone-protocol/contracts/commit/6a8d7b94c74d17245a3de3c2f0acd26e6a44949f))
* **cognitarium:** design insert execute message ([3994205](https://github.com/axone-protocol/contracts/commit/39942050c1f07df9b59315c1882a4602e1b1b921))
* **cognitarium:** design instantiate message ([1edacf6](https://github.com/axone-protocol/contracts/commit/1edacf6748b2b1fd0a17e801e1336fd608f115ef))
* **cognitarium:** design query select resources message ([3934ddf](https://github.com/axone-protocol/contracts/commit/3934ddfa61a2d20ddd9f7a159eecd37def971c5b))
* **cognitarium:** design remove execute message ([bb901da](https://github.com/axone-protocol/contracts/commit/bb901dafda75dc1e88aa746c9d18c4117ff5208e))
* **cognitarium:** design triples state ([a38aa79](https://github.com/axone-protocol/contracts/commit/a38aa79752c4e1162eff5fd3a23c9c65c6612ced))
* **cognitarium:** disallow select with no where clause ([bf89eaa](https://github.com/axone-protocol/contracts/commit/bf89eaa1264253c171cdc805f750d6f3fea644a1))
* **cognitarium:** handle engine limit query node ([9fa3b9d](https://github.com/axone-protocol/contracts/commit/9fa3b9da23e8d4f5969a4052de4bb01d92b5c37b))
* **cognitarium:** handle limit & skip at plan build ([a96258f](https://github.com/axone-protocol/contracts/commit/a96258fa688d9b66a25895d31c9f700b2d6debd1))
* **cognitarium:** handle query engine for loop join ([f9af316](https://github.com/axone-protocol/contracts/commit/f9af31624f5cac5aa14956a29ed3896404cf1d27))
* **cognitarium:** handle query engine skip node ([65e09ee](https://github.com/axone-protocol/contracts/commit/65e09ee51c28585280db920aa5ac1c4f19e64d0f))
* **cognitarium:** impl map query var to msg value ([bc8ed42](https://github.com/axone-protocol/contracts/commit/bc8ed42bda4baa3aafd978c9c198c498b0bb1704))
* **cognitarium:** implement base insert logic ([79dbc9b](https://github.com/axone-protocol/contracts/commit/79dbc9bf7304f549e99bb92ebfff424d2e5087c9))
* **cognitarium:** implement instantiate msg ([05e6319](https://github.com/axone-protocol/contracts/commit/05e63191d85565faa1a56cb2f69eaf5300ecb6ef))
* **cognitarium:** implement rdf parsing ([1f8c0d0](https://github.com/axone-protocol/contracts/commit/1f8c0d085433648172e94056b4ab167ac1a722a1))
* **cognitarium:** implement store configuration state ([8b4e10b](https://github.com/axone-protocol/contracts/commit/8b4e10b423b64ffe24d2fc2456db85ccb3c6ea92))
* **cognitarium:** implement store query msg ([4f0b8fb](https://github.com/axone-protocol/contracts/commit/4f0b8fb567073b616ee301419d592b17cd48e25f))
* **cognitarium:** implement triple mapping with rio api ([c13f530](https://github.com/axone-protocol/contracts/commit/c13f530a7c0fcb22271748d4f018a0e71ac6832a))
* **cognitarium:** implements building triple pattern query node ([7f32451](https://github.com/axone-protocol/contracts/commit/7f32451ef530cb286b05d0ec80fe72b42f6ad597))
* **cognitarium:** implements cosmwasm primary key serde for triple ([4f19584](https://github.com/axone-protocol/contracts/commit/4f195846744a5c5a364a4d71d802721a9b2b2fd2))
* **cognitarium:** implements query engine cartesian join ([112d07f](https://github.com/axone-protocol/contracts/commit/112d07f1134d632aab38046152d9ed9849d506b9))
* **cognitarium:** implements query engine main logic ([ee8254b](https://github.com/axone-protocol/contracts/commit/ee8254b51e00611f4858f173a742582ac012f37e))
* **cognitarium:** implements query resovled var mappings ([6d589c7](https://github.com/axone-protocol/contracts/commit/6d589c771013147b81be5c578017f5ecfabc73cf))
* **cognitarium:** implements select query ([bf8c45d](https://github.com/axone-protocol/contracts/commit/bf8c45d46fcf4c821a1634e7164c07febf425088))
* **cognitarium:** implements triple pattern iterator ([7c1c800](https://github.com/axone-protocol/contracts/commit/7c1c800a3ab2e053eba80baad1d3dc6031661a71))
* **cognitarium:** introduce query plan model ([3a074be](https://github.com/axone-protocol/contracts/commit/3a074bee14163e8c7762ace7e687119025361246))
* **cognitarium:** maintain namespace counter in state ([24e4c45](https://github.com/axone-protocol/contracts/commit/24e4c450e56e05fea0fbca593ed4a41d3f084469))
* **cognitarium:** make linter happy ([b07861a](https://github.com/axone-protocol/contracts/commit/b07861a4e4f2f4700a4e6253ef6ca15922eb9bee))
* **cognitarium:** manage insert error cases ([c10cf6a](https://github.com/axone-protocol/contracts/commit/c10cf6af731e73f2967d1339bf5d4f1ed4f6cb32))
* **cognitarium:** perform authorization before insert ([9c19cf3](https://github.com/axone-protocol/contracts/commit/9c19cf38b6d89e03b280908cd5d549bf3b0b795e))
* **cognitarium:** resolve query node bound variables ([dd6953f](https://github.com/axone-protocol/contracts/commit/dd6953f593b3eace5511db573870c7f990afc202))
* **cognitarium:** rework querying interface ([1213e38](https://github.com/axone-protocol/contracts/commit/1213e38c9877d88210c795e24d3e12db022998a2))
* **cognitarium:** rework triples primary key storage ([5409300](https://github.com/axone-protocol/contracts/commit/5409300bcef438d5b9be0da7de62baead8ace99c))
* **cognitarium:** separate iri namespaces from triples ([a5066ea](https://github.com/axone-protocol/contracts/commit/a5066eaf6a5680a5b365a5bbd0671fde9dcc7050))
* **cognitarium:** specify the sparql select query ([b4ed270](https://github.com/axone-protocol/contracts/commit/b4ed270cc691b80c9310c09871eec5979d094b20))
* **cognitarium:** specify Describe query ([158ed40](https://github.com/axone-protocol/contracts/commit/158ed40ed2cd8290d46af45232fdbd103e009ba0))
* **cognitarium:** specify max_byte_size limit ([62c2f81](https://github.com/axone-protocol/contracts/commit/62c2f81fb690a2313d676862bb7fde5ffdd8a450))
* **cognitarium:** specify max_insert_data_byte_size limit ([5c46dd8](https://github.com/axone-protocol/contracts/commit/5c46dd803155991f54d28684692ab1e35d2ab0ca))
* **cognitarium:** specify max_insert_data_triple_count limit ([47fd466](https://github.com/axone-protocol/contracts/commit/47fd466446cb010b24a4b5c459d82b9f3fa422dd))
* **cognitarium:** specify max_triple_byte_size limit ([a467889](https://github.com/axone-protocol/contracts/commit/a467889caf988dcbcba160f494e3b18af501effc))
* **cognitarium:** specify prefixes support for select query ([4568e49](https://github.com/axone-protocol/contracts/commit/4568e49ef909b745dbaf2bcc70fe8608bffc75dc))
* **cognitarium:** specify rdf triple deletion ([bc1e31e](https://github.com/axone-protocol/contracts/commit/bc1e31e84433a9a7e32986210209d4e6543f6fae))
* **cognitarium:** specify the max_query_variable_count limit ([2b0231d](https://github.com/axone-protocol/contracts/commit/2b0231db4473df5aeddd9b5c45c55343d7d77999))
* **cognitarium:** update query limit msg type ([eb3353b](https://github.com/axone-protocol/contracts/commit/eb3353bde7f4251492ab09cd99c6934fc1ae1d75))
* **coqgnitarium:** implement select at query engine level ([5a85ef3](https://github.com/axone-protocol/contracts/commit/5a85ef36e9b789f5232c2f67dcf0402f790242ad))
* **logic-bindings:** implements term value parser ([888eaf9](https://github.com/axone-protocol/contracts/commit/888eaf94386f302f16acc1958d5d3b07adfec4a2))
* **logic-bindings:** prune mocking elements ([49157aa](https://github.com/axone-protocol/contracts/commit/49157aa11ecae992ceb26c4b3c79219f5a9b3a02))
* **logic:** implements logic cosmwasm URI handling ([c539bf5](https://github.com/axone-protocol/contracts/commit/c539bf5744eceb80eb773ac74d6b2900fb75afd6))
* **objectarium:** add compression functions (partially) ([5ace7a3](https://github.com/axone-protocol/contracts/commit/5ace7a393242f8dccc6332bca166b5c274d88cb6))
* **objectarium:** implement compression of objects ([36c5068](https://github.com/axone-protocol/contracts/commit/36c50685d7f6c6ea2cf1756c83dfc49681be771c))
* **objectarium:** implement MD5 hash algorithm ([be4bb16](https://github.com/axone-protocol/contracts/commit/be4bb16c313947962ed801d2861d2ac6d0b85776))
* **objectarium:** implement SHA-224 hash algorithm ([3f8b938](https://github.com/axone-protocol/contracts/commit/3f8b93839839355311900164324cbd628e82f3fb))
* **objectarium:** implement SHA-384 hash algorithm ([36e5e05](https://github.com/axone-protocol/contracts/commit/36e5e050c072535f1fa9ee3daaa3497ab2cc0bf5))
* **objectarium:** implement SHA-512 hash algorithm ([550d87f](https://github.com/axone-protocol/contracts/commit/550d87fd00c6f8bdbf9cc29dbbfb057d72c318b9))
* **objectarium:** implement Snappy compression algorithm ([2958730](https://github.com/axone-protocol/contracts/commit/29587308b8247d7128fa7d485395b4116938ec7d))
* **objectarium:** specify compression support for objects ([d84cc5e](https://github.com/axone-protocol/contracts/commit/d84cc5e885d1038c77c6a4f2e5b1059506fae928))
* **objectarium:** update objectarium sample according to api change ([db61108](https://github.com/axone-protocol/contracts/commit/db61108af2345b450730d9197c69855212f651d0))
* **storage:** implements storage object logic fs URI ([42d709d](https://github.com/axone-protocol/contracts/commit/42d709d5bda637cd5e61b87580b205f7ec66733e))
* **storage:** map object ref to cw-storage msgs ([dfa6554](https://github.com/axone-protocol/contracts/commit/dfa6554eb10a8944ca7b11c1fd180469da31e60c))


### Bug Fixes

* **cognitarium:** avoid opening triple iter with wrong variables type ([fbd8bc4](https://github.com/axone-protocol/contracts/commit/fbd8bc4c9e6fbd2af3c05903ed5af81452e3dca3))
* **cognitarium:** ensure data input property naming ([87e93d4](https://github.com/axone-protocol/contracts/commit/87e93d4082b72d390fc5a67a208716dcc2cfb080))
* **cognitarium:** extraction of prefix ([9393523](https://github.com/axone-protocol/contracts/commit/9393523f82646d13654915f5cdcdf69f8a84c5a7))
* **cognitarium:** handle urn in explode iri ([3837b8e](https://github.com/axone-protocol/contracts/commit/3837b8e5f96d0366b0db0a059d1aad62df6a17b0))
* **cognitarium:** make the query engine compile! ([23536ae](https://github.com/axone-protocol/contracts/commit/23536ae3362eadaf4c79f4d42113957e416fd1fe))
* **cognitarium:** manage default values for store limits ([148fdef](https://github.com/axone-protocol/contracts/commit/148fdefd77fdd72164ff73ca79eca135f9774d89))
* **cognitarium:** manage non implemented messages ([67fdc69](https://github.com/axone-protocol/contracts/commit/67fdc690517a7cad7918df4cc70c7bb74f6d809e))
* **cognitarium:** properly chose join type ([3ff77df](https://github.com/axone-protocol/contracts/commit/3ff77df605e85f44fd0d7739ac22a503c93f01bd))
* **cognitarium:** properly format output json keys ([c4de274](https://github.com/axone-protocol/contracts/commit/c4de27469f013041ce451eecfef09c5c8f0b8da0))
* **cognitarium:** return empty iter on not found triple iter load ([757177d](https://github.com/axone-protocol/contracts/commit/757177d340e1748a6ac043376598e8e419e90acb))
* **cognitarium:** use binary representation of triple pk parts ([30ae1f4](https://github.com/axone-protocol/contracts/commit/30ae1f43704d1c68ffa25836898d4253e5ea6177))
* **cognitarium:** use btree instead of hashmap for select results ([01a052d](https://github.com/axone-protocol/contracts/commit/01a052d87090d9d4a3b91f285deb3c3207c9b42e))
* **docs:** generate docs escaping invalid char ([1f9f717](https://github.com/axone-protocol/contracts/commit/1f9f717bd60c4fa1b501afae4c8ced89490c7088))
* **objectarium:** remove object data on forget_object ([7ee621d](https://github.com/axone-protocol/contracts/commit/7ee621de8aee46251a9ed8e294da6ef4704708fa))
* set correct contract name for contracts (based on crate name) ([f52db19](https://github.com/axone-protocol/contracts/commit/f52db1948ecac9e8fa6da3a54ecd6bc312d6bfd7))

## 1.0.0 (2023-03-24)


### Features

* add bucket query ([4d9976d](https://github.com/axone-protocol/contracts/commit/4d9976d316ca0d327dc70947f133accf6800d18d))
* add cw-template ([72d943e](https://github.com/axone-protocol/contracts/commit/72d943e373e35e6679b94cc547b45c55dfe170e3))
* **ci:** add update draft doc trigger ([8ee54dd](https://github.com/axone-protocol/contracts/commit/8ee54dd33bae17245d5b254dc6554619c779d769))
* **cw-storage:** add bucket limits ([604086f](https://github.com/axone-protocol/contracts/commit/604086f56e12f32317d5760c3894ac1d505fcff4))
* **cw-storage:** add ObjectData query ([a022a00](https://github.com/axone-protocol/contracts/commit/a022a00435bf65d015f4c720453d70b58571cf07))
* **cw-storage:** specify contract ([2080e5c](https://github.com/axone-protocol/contracts/commit/2080e5c9ba628ba3201a23277db9e0e5ba929393))
* **docs:** add documentations title on attribute ([48cc0b9](https://github.com/axone-protocol/contracts/commit/48cc0b9b62564a4347628ea976ebbda743af3d01))
* **docs:** add script to improve docs description ([b26698c](https://github.com/axone-protocol/contracts/commit/b26698cc8df812a9ef88f3311982f418e0bd93ea))
* **docs:** change struct comment to be displayed on generated docs ([7bdf548](https://github.com/axone-protocol/contracts/commit/7bdf548c175caec05e3294f84dce3189e6badadd))
* **docs:** check if jsonschema2md is present ([70bc47b](https://github.com/axone-protocol/contracts/commit/70bc47bb4ad1a6ece5d557be8388307684d0d763))
* **docs:** fail generate docs if jsonschema2md is not installed ([c22e902](https://github.com/axone-protocol/contracts/commit/c22e9020cc66a9d4cbad37a58021a12dd268368d))
* **docs:** generate documentation ([a88941d](https://github.com/axone-protocol/contracts/commit/a88941d624a3cc20e16668eb9299310dce7107ab))
* **docs:** generate documentation with jsonschema2md ([5b165e3](https://github.com/axone-protocol/contracts/commit/5b165e3e1a99b17f29f26b9325cccbe1ab439fc1))
* **docs:** ignore docs folder from markdown linter ([4c7011d](https://github.com/axone-protocol/contracts/commit/4c7011df3e16844674d039383204c43f3dc16a39))
* **docs:** lint changed docs files ([068a719](https://github.com/axone-protocol/contracts/commit/068a719fe0851bbdd30896730e2b849f00d6a8ca))
* **docs:** mulitple schema generation ([958093b](https://github.com/axone-protocol/contracts/commit/958093be1f190a30d95d8b85360d1fe74172bfc5))
* **docs:** only ignore json schema on contract folder ([6a43254](https://github.com/axone-protocol/contracts/commit/6a43254aafb2bc34339466fb2e2087c2d02d90eb))
* **docs:** publish docs ([63c533b](https://github.com/axone-protocol/contracts/commit/63c533bc5a9e21a0799d29f2f6492b3be7a6420f))
* **docs:** remove schema folder from git ([8eae25d](https://github.com/axone-protocol/contracts/commit/8eae25dea8dfbaf373313aca3e3e92592c8fd8e8))
* **docs:** rm docs folder before generation ([65ddb99](https://github.com/axone-protocol/contracts/commit/65ddb993e82ac7861393f5fc566d52822be0fc8d))
* **docs:** trigger the docs workflow to update documentation ([e50e401](https://github.com/axone-protocol/contracts/commit/e50e401a1c55382858a5182d69633b76d47cbde9))
* **gov:** add query retrieving program location ([7ab021f](https://github.com/axone-protocol/contracts/commit/7ab021fd9251e515031a2495f036b48cb873db18))
* **gov:** design contract messages ([bb54756](https://github.com/axone-protocol/contracts/commit/bb54756bb7b22425f605d09c67d6364488744d72))
* **law:** add break stone exec signature ([263e6db](https://github.com/axone-protocol/contracts/commit/263e6db72df195668497400a77102e51e87cf159))
* **law:** add broken flag in state ([f82073f](https://github.com/axone-protocol/contracts/commit/f82073fbde72e18cf49349367dc7a6e93f669216))
* **law:** add Law state + tests ([35e8e91](https://github.com/axone-protocol/contracts/commit/35e8e9199742f27aff5dde678cf19bf67f66bdde))
* **law:** add uri parser to Object ([3ec96b2](https://github.com/axone-protocol/contracts/commit/3ec96b210ec7b5bfa293b4007024f08871ef5452))
* **law:** check if law is broken before ask ([0cdf648](https://github.com/axone-protocol/contracts/commit/0cdf64868fc200cb36b445c5be94c497a030ed23))
* **law:** create temporary context on instantiate ([e9de097](https://github.com/axone-protocol/contracts/commit/e9de097d6f013663404d695594fd9841dff2dc3c))
* **law:** impl Ask query ([af37eb7](https://github.com/axone-protocol/contracts/commit/af37eb7fd213912cce571bb093a103267ed87371))
* **law:** impl from<Object> on ObjectResponse ([17e31d1](https://github.com/axone-protocol/contracts/commit/17e31d1b7f6620c4846adf5ff6c8ac17519b703c))
* **law:** impl into StdError on ContractError ([d59b268](https://github.com/axone-protocol/contracts/commit/d59b2685fa029a0088d1252ea666c39bfc0c3306))
* **law:** implement instantiate ([2adaa5b](https://github.com/axone-protocol/contracts/commit/2adaa5b76009bb036f4cbc8002985a20a8bcd32b))
* **law:** implement the program query ([fe4b0da](https://github.com/axone-protocol/contracts/commit/fe4b0da6ac5aae0f36cd264279a5e6eba593ba44))
* **law:** implements break stone exec msg ([c28535f](https://github.com/axone-protocol/contracts/commit/c28535f3ea8fba9222d2fd1cf13df73cd1c58a90))
* **law:** implements object to uri mapping ([06df729](https://github.com/axone-protocol/contracts/commit/06df729421747dcc81148b620f072287115da362))
* **law:** implements source_files ask query ([c27c2db](https://github.com/axone-protocol/contracts/commit/c27c2db1d5e2338a34513b1bdde403419e98fe19))
* **law:** instantiate with sub message store ([8819c38](https://github.com/axone-protocol/contracts/commit/8819c3824d823d120915176e158d9db088cec9ed))
* **law:** update state with object and dependencies ([314bf4a](https://github.com/axone-protocol/contracts/commit/314bf4aa09003b485ae90a93c8697dd72e52842a))
* **logic:** add a sample logic query contract ([8a67c8d](https://github.com/axone-protocol/contracts/commit/8a67c8d530d13541ee1fd7e452c630a76825b052))
* **logic:** introduce logic module bindings ([465a4fe](https://github.com/axone-protocol/contracts/commit/465a4fe30f20f94536e11ec46d446c5fa8fcceb1))
* remove main.rs ([62090ca](https://github.com/axone-protocol/contracts/commit/62090ca74657f35058c5455ed6c51ea397fd3a98))
* **schema:** generate schema not at workspace place ([c1b1f28](https://github.com/axone-protocol/contracts/commit/c1b1f28c5babc9fd52bacac2eccc8460d082ef3f))
* **storage:** add cursor bs58 encoding helpers ([1e7c2bc](https://github.com/axone-protocol/contracts/commit/1e7c2bc07594143bc18e9f970dda704452a14fe4))
* **storage:** add execute PinObject ([07f5e37](https://github.com/axone-protocol/contracts/commit/07f5e378f070cdef427806edfda78bc2a1444b6b))
* **storage:** add pagination configuration ([6eeb157](https://github.com/axone-protocol/contracts/commit/6eeb157902ad946f5ec1d411df5e994375d09979))
* **storage:** add pin count attribute on Object ([04d8bff](https://github.com/axone-protocol/contracts/commit/04d8bfff8cdfb9451a94a802fbcb205c414077e4))
* **storage:** add queries dummy impl ([23f6de9](https://github.com/axone-protocol/contracts/commit/23f6de9d130bfa56d6a0841b4f4c7857040c5b2d))
* **storage:** add sha256 hash computation util ([ada8714](https://github.com/axone-protocol/contracts/commit/ada8714eaeb2589c3c3442b03096f2477ac4ccfc))
* **storage:** add the query to get bucket information ([35b6184](https://github.com/axone-protocol/contracts/commit/35b6184c9478c53b44f08bd645ba319072c73184))
* **storage:** add total size to bucket state ([ce5d622](https://github.com/axone-protocol/contracts/commit/ce5d62291e2e98666ada3b7344bc3c10c8479034))
* **storage:** check if bucket name is not empty ([eb193db](https://github.com/axone-protocol/contracts/commit/eb193db394143dfdaa3ebbc2fadc5788e8c9ae03))
* **storage:** create bucket state ([53e2b8c](https://github.com/axone-protocol/contracts/commit/53e2b8c2ac5bc9c133c3076b04bc913a97b7689c))
* **storage:** define errors related to bucket limits ([4c6587d](https://github.com/axone-protocol/contracts/commit/4c6587d6c125578002dc29b44cd8cb48be66864d))
* **storage:** enhance error context ([c7e33d8](https://github.com/axone-protocol/contracts/commit/c7e33d813d162cc50bac941751ee0802efa01e39))
* **storage:** implement the forget object execute message ([52f1666](https://github.com/axone-protocol/contracts/commit/52f16661c60c70a4f608c1913b48693181cd0cb3))
* **storage:** implement unpin object ([94613c5](https://github.com/axone-protocol/contracts/commit/94613c5dab60ba8a837aec9ca5985948c1f4b822))
* **storage:** implements data state structure ([0212af3](https://github.com/axone-protocol/contracts/commit/0212af383e0f80bfe71a60e50283f69f0d6a813f))
* **storage:** implements object data query ([11267fa](https://github.com/axone-protocol/contracts/commit/11267fa6009b89540705af7aa192eb02699f073e))
* **storage:** implements object pins query ([af9d04f](https://github.com/axone-protocol/contracts/commit/af9d04ffaebc3f875e59bcf13c076cbf191706ed))
* **storage:** implements object query ([5bccebd](https://github.com/axone-protocol/contracts/commit/5bccebd744d3bf4aa96f3420f8ee7f8ea9cce47a))
* **storage:** implements objects query ([4326b4c](https://github.com/axone-protocol/contracts/commit/4326b4c539ec4f3ae275de930438f246b7e7623e))
* **storage:** implements objects state structure ([cf67f3e](https://github.com/axone-protocol/contracts/commit/cf67f3e6d77ce386d8572249b7605a447ffe4d8a))
* **storage:** implements pins state structure ([467c2da](https://github.com/axone-protocol/contracts/commit/467c2daf50cf827d67473aee16c1365e092e337f))
* **storage:** implements store object exec msg ([8a1b618](https://github.com/axone-protocol/contracts/commit/8a1b618598507d98ccf62f7431091613b809d2e4))
* **storage:** instantiate bucket ([c56df54](https://github.com/axone-protocol/contracts/commit/c56df546f5470dd63c5b60a3774a8e3db419dc31))
* **storage:** load buckect only if object exist ([57327db](https://github.com/axone-protocol/contracts/commit/57327db4a4f7cc28690680dcfd9b8f227d13bbeb))
* **storage:** persist bucket owner address ([758ed9a](https://github.com/axone-protocol/contracts/commit/758ed9a7fe1e3f0b8a91f5e54ed6bfc1605ad73c))
* **storage:** persist object count in bucket state ([e233791](https://github.com/axone-protocol/contracts/commit/e2337916afb482ba1380bcbb8161c332c1fe3a11))
* **storage:** remove address index on pins state ([05e07ec](https://github.com/axone-protocol/contracts/commit/05e07ec3099881a49fc9b9dce3f19423b7927e25))
* **storage:** remove unused NotImplemented error ([7cbb7c6](https://github.com/axone-protocol/contracts/commit/7cbb7c6f864aa23199a91358fda4cee8e856ec87))
* **storage:** remove whitespace from bucket name ([403451b](https://github.com/axone-protocol/contracts/commit/403451b861e40f02832c7c70b93c859d33d9803c))
* **storage:** return error when storing already stored object ([da89b6f](https://github.com/axone-protocol/contracts/commit/da89b6fd7afd32f4aabf4660a5c6bfd7376d4ba1))
* **storage:** try to generify pagination logic ([e229988](https://github.com/axone-protocol/contracts/commit/e2299887cc1748a02f4fb3d17628c00f08cbb5f0))
* **storage:** unpin sender on forget ([9f027ca](https://github.com/axone-protocol/contracts/commit/9f027cac3e7569ab5300c99748bddb6edb494d01))
* **template:** add task to generate schema ([815c58e](https://github.com/axone-protocol/contracts/commit/815c58e5c1746b994ca522150f8207c4c5f30531))
* **template:** build asC lib for generate wasm ([9e9f5c9](https://github.com/axone-protocol/contracts/commit/9e9f5c9211fd926d5d9765ec247f30b9a64bd460))
* **template:** format rust and toml ([e4dadf1](https://github.com/axone-protocol/contracts/commit/e4dadf1d3682dbf8aad968e345a8b56489d68e86))
* **template:** update template with new from cosmwasm ([9656cca](https://github.com/axone-protocol/contracts/commit/9656cca1e7b58aac9d58c45032b1608abab023f9))


### Bug Fixes

* **ci:** install jsonschema through yarn global and include in path ([16b88bc](https://github.com/axone-protocol/contracts/commit/16b88bc2400d65205f9d904236b6a80bd03037ef))
* **ci:** only trigger report wasm size on PR ([deecbd1](https://github.com/axone-protocol/contracts/commit/deecbd12cd3e735013fca73b589c0f06c46c9b57))
* **ci:** remove check feature on test and lint ([6a431e1](https://github.com/axone-protocol/contracts/commit/6a431e19dbcad1dbb25862ccc00ed8069758b22f))
* **ci:** tarpaulin cargo crate name ([9d1931c](https://github.com/axone-protocol/contracts/commit/9d1931c9f5ddc4337d4a37f15d15606beebc9ac8))
* **ci:** trigger the good workflow id ([e611394](https://github.com/axone-protocol/contracts/commit/e6113944332bdfafcf96f647aefe4843fe7e6264))
* **ci:** use bash shell for check_contract task ([eded5b1](https://github.com/axone-protocol/contracts/commit/eded5b13fdb2840d3da68815d6f809fe4e20b537))
* **ci:** use new check-contract crates ([58cdf37](https://github.com/axone-protocol/contracts/commit/58cdf376a5cb6d76b1a692925c94d1f6e0d3693f))
* **ci:** verify docs update ([12eb19e](https://github.com/axone-protocol/contracts/commit/12eb19e4990d4950e7b9f9516e7786e34788a659))
* **ci:** wrong object input for trigger workflow ([ee5fe8d](https://github.com/axone-protocol/contracts/commit/ee5fe8d6820e68da61ebfb4eadac570be21f4560))
* **docs:** correctly escaping error message ([c9b61e8](https://github.com/axone-protocol/contracts/commit/c9b61e8e42d62a30c0d1a3868eeacc8fe54e22af))
* **docs:** disable checking jsonschema2md binary ([dfcf049](https://github.com/axone-protocol/contracts/commit/dfcf049df196356a52877b785b9608347c32249f))
* **docs:** only find json schema at root folder ([7145f6e](https://github.com/axone-protocol/contracts/commit/7145f6edd61ac709d9c89b920c5f8683052677bc))
* **docs:** search json file into schema folder for gen doc ([c48bc23](https://github.com/axone-protocol/contracts/commit/c48bc239a71be72d1ca0f8efc66b2d2bbf798a4f))
* format code ([1fd1571](https://github.com/axone-protocol/contracts/commit/1fd1571b5aa263d3ad92a9650a9f5bb0df3dfbb1))
* **law:** linter impl From unstead of into ([49bc80c](https://github.com/axone-protocol/contracts/commit/49bc80c1a08edf9ffa664ac446bc5485b275def3))
* **law:** program conversion base64 to string ([3b11d2d](https://github.com/axone-protocol/contracts/commit/3b11d2dfca1d8ab6981f70791cd57378b1946ae7))
* **lint:** derive Eq when derive PartialEq ([28df736](https://github.com/axone-protocol/contracts/commit/28df7368832bbeae28ea6012ee44454163cb3fba))
* **lint:** format all rust code ([b955d74](https://github.com/axone-protocol/contracts/commit/b955d745ab2cbf8c8edf2fe67b15df0a205cca8e))
* **lint:** simplify assert ([fd08d58](https://github.com/axone-protocol/contracts/commit/fd08d5890e680c3a22eccbf49c9f120ba33cd20d))
* **lint:** update cw-template readme ([79629bb](https://github.com/axone-protocol/contracts/commit/79629bb43598595073cc5b10aa2679bb0f1eec18))
* **lint:** update readme by cleaning template stuff ([3137e2a](https://github.com/axone-protocol/contracts/commit/3137e2ac180775ead2ef6bf6b3312b45a740e1bb))
* make linter happy ([241e6ec](https://github.com/axone-protocol/contracts/commit/241e6ec096a2a30fc36b50fd33df11391824e2d6))
* **storage:** ensure error querying pins on unexisting object ([7e269ed](https://github.com/axone-protocol/contracts/commit/7e269ed7bd896d5f689f8bbbc346fdf3371f3c4a))
* **storage:** ensure max objects limit ([0734b58](https://github.com/axone-protocol/contracts/commit/0734b58ca1a6e64e2a6ac40e35b25f9b86985da5))
* **storage:** indicate max pin count in error ([4bb1af0](https://github.com/axone-protocol/contracts/commit/4bb1af05acb5fe16de250b770666fd8569bb398c))
* **storage:** lint and format ([2b9cfa2](https://github.com/axone-protocol/contracts/commit/2b9cfa27f58a7b721ea1cdd40511159397592192))
* **storage:** make PageInfo unidirectionnal ([9245e30](https://github.com/axone-protocol/contracts/commit/9245e300d818af07fb3e43764e6bd7f0e585cdee))
* **storage:** solve object query is_pinned ([3f26651](https://github.com/axone-protocol/contracts/commit/3f26651895066cd601868348497ef894c22900c3))
* **storage:** use Uint128 instead of primitive in state ([803e1b3](https://github.com/axone-protocol/contracts/commit/803e1b367ee2185b923b398b96e8eef39ead3e63))
* **test:** apply linter new rules ([43129c0](https://github.com/axone-protocol/contracts/commit/43129c06bc1c199e9c58bf3413a325859734f744))
