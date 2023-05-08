:robot: I have created a release *beep* *boop*
---


<details><summary>noosphere: 0.11.0</summary>

## [0.11.0](https://github.com/cdata/noosphere/compare/noosphere-v0.10.2...noosphere-v0.11.0) (2023-05-08)


###   BREAKING CHANGES

* Revised tracing configuration ([#342](https://github.com/cdata/noosphere/issues/342))
* Some non-blocking, callback-based C FFI ([#322](https://github.com/cdata/noosphere/issues/322))
* Sphere traversal C FFI ([#292](https://github.com/cdata/noosphere/issues/292))
* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284))
* Fork `iroh-car` as `noosphere-car` ([#283](https://github.com/cdata/noosphere/issues/283))
* Implement C FFI for petname management ([#271](https://github.com/cdata/noosphere/issues/271))
* Petname resolution and synchronization in spheres and gateways ([#253](https://github.com/cdata/noosphere/issues/253))
* Reconfigure module dependencies so that noosphere-ipfs depends on noosphere-storage, and not the other way around creating a cycle. ([#254](https://github.com/cdata/noosphere/issues/254))
* Templatize the two IPFS HTTP APIs as noosphere_ipfs::IpfsClient, and reconfigure KuboStorage as IpfsStorage, operating on IpfsClient rather than a URL. ([#252](https://github.com/cdata/noosphere/issues/252))
* Sphere sync and change diff in C FFI ([#210](https://github.com/cdata/noosphere/issues/210))
* `SphereFile` fields referring to a `revision` now refer to a `version` instead.
* Several critical dependencies of this library were updated to new versions that contain breaking changes.
* The `StorageProvider` trait has been replaced by the `Storage` trait. This new trait allows for distinct backing implementations of `BlockStore` and `KeyValueStore`.
* The `.sphere` directory has a new layout; the files previously used to store metadata have been replaced with database metadata; the `blocks` directory is now called `storage`. At this time the easiest migration path is to initialize a new sphere and copy your existing files into it.
* `SphereIpld` identity is now a `Did`
* Some FFI interfaces now have simplified interfaces.
* Many APIs that previously asked for bare strings when a DID string was expected now expect a newtype called `Did` that wraps a string.
* The `noosphere-api` Client now holds an owned key instead of a reference.

### Features

* `SphereFs` is initialized with key material ([#140](https://github.com/cdata/noosphere/issues/140)) ([af48061](https://github.com/cdata/noosphere/commit/af4806114ca8f7703e0a888c7f369a4a4ed69c00))
* Add `noosphere` crate-based Swift package ([#131](https://github.com/cdata/noosphere/issues/131)) ([e1204c2](https://github.com/cdata/noosphere/commit/e1204c2a5822c3c0dbb7e61bbacffb2c1f49d8d8))
* Add `ns_error_code_get()` to FFI. Fixes [#332](https://github.com/cdata/noosphere/issues/332) ([#340](https://github.com/cdata/noosphere/issues/340)) ([4156328](https://github.com/cdata/noosphere/commit/41563288150725e87f3891abce15966220d92177))
* Add `SphereFS` read/write to FFI ([#141](https://github.com/cdata/noosphere/issues/141)) ([26e34ac](https://github.com/cdata/noosphere/commit/26e34acfe70cac099acfa6dc8c2cf156c46fdae0))
* Beautify the Sphere Viewer demo app ([#186](https://github.com/cdata/noosphere/issues/186)) ([3e30fdb](https://github.com/cdata/noosphere/commit/3e30fdb5e2b6758397f05343491a36512a4f4a0c))
* Consider creating a new key with an empty string an error. Fixes [#331](https://github.com/cdata/noosphere/issues/331) ([#354](https://github.com/cdata/noosphere/issues/354)) ([0a0efa6](https://github.com/cdata/noosphere/commit/0a0efa60be5f258476249d5d8c8d5fb93911c42e))
* Dot syntax when traversing by petname ([#306](https://github.com/cdata/noosphere/issues/306)) ([cd87b05](https://github.com/cdata/noosphere/commit/cd87b0533c21bbbd4d82332556e70ecc706a5531))
* Enable expired yet valid records in the name system. Update to ucan 0.2.0. ([#360](https://github.com/cdata/noosphere/issues/360)) ([3b0663a](https://github.com/cdata/noosphere/commit/3b0663abc7783a6d33dd47d20caae7597ab93ed0))
* Extend C FFI for header enumeration ([#202](https://github.com/cdata/noosphere/issues/202)) ([b404ec0](https://github.com/cdata/noosphere/commit/b404ec0d117e2467bfbe4a3bda4253e1c57f584e))
* Fork `iroh-car` as `noosphere-car` ([#283](https://github.com/cdata/noosphere/issues/283)) ([b0b7c38](https://github.com/cdata/noosphere/commit/b0b7c3835ff1ef271bbe0f833f6f7856fcc30de1))
* General error handling in C FFI ([#219](https://github.com/cdata/noosphere/issues/219)) ([0a1952b](https://github.com/cdata/noosphere/commit/0a1952b34895071d2203505c95750d453bb110c6))
* Implement C FFI for petname management ([#271](https://github.com/cdata/noosphere/issues/271)) ([d43c628](https://github.com/cdata/noosphere/commit/d43c6283c6b2374de503d70bd46c8df7d0337c3a))
* Initial example of C integration. ([#242](https://github.com/cdata/noosphere/issues/242)) ([57beb24](https://github.com/cdata/noosphere/commit/57beb24f9996a92fa348657a58920a7944f53e05))
* Introduce `noosphere-gateway` crate ([#238](https://github.com/cdata/noosphere/issues/238)) ([791bc39](https://github.com/cdata/noosphere/commit/791bc3996cfac12cb077c3721f22d080a71d33ba))
* Introduce `ns_sphere_identity` FFI call ([#317](https://github.com/cdata/noosphere/issues/317)) ([81f9c3b](https://github.com/cdata/noosphere/commit/81f9c3bb5e861d601d86326c80ffc48c0d875c7e))
* Introduce pet names to spheres ([#154](https://github.com/cdata/noosphere/issues/154)) ([7495796](https://github.com/cdata/noosphere/commit/74957968af7f7e51a6aa731192431fbf5e01215e))
* Introduce web bindings and `orb` NPM package ([#182](https://github.com/cdata/noosphere/issues/182)) ([44170a2](https://github.com/cdata/noosphere/commit/44170a27be2e1d180b1cee153937ab2cef16a591))
* Noosphere builds and runs tests on Windows ([#228](https://github.com/cdata/noosphere/issues/228)) ([d1320f0](https://github.com/cdata/noosphere/commit/d1320f08429c8f8090fd4612b56ebf9386414cc7))
* **noosphere:** Introduce `noosphere` crate ([#123](https://github.com/cdata/noosphere/issues/123)) ([ad9daa6](https://github.com/cdata/noosphere/commit/ad9daa697067069197d12ee8e7f11bdbedc3662d))
* Petname resolution and synchronization in spheres and gateways ([#253](https://github.com/cdata/noosphere/issues/253)) ([f7ddfa7](https://github.com/cdata/noosphere/commit/f7ddfa7b65129efe795c6e3fca58cdc22799127a))
* Publish name record from gateway periodically. ([#334](https://github.com/cdata/noosphere/issues/334)) ([fc5e42f](https://github.com/cdata/noosphere/commit/fc5e42f2bd918fc1b3c448e55c611a99d49b00db))
* Re-implement `noosphere-cli` in terms of `noosphere` ([#162](https://github.com/cdata/noosphere/issues/162)) ([1e83bbb](https://github.com/cdata/noosphere/commit/1e83bbb689642b878f4f6909d7dd4a6df56b29f9))
* Refactor storage interfaces ([#178](https://github.com/cdata/noosphere/issues/178)) ([4db55c4](https://github.com/cdata/noosphere/commit/4db55c4cba56b329a638a4227e7f3247ad8d319c))
* Revised tracing configuration ([#342](https://github.com/cdata/noosphere/issues/342)) ([c4a4084](https://github.com/cdata/noosphere/commit/c4a4084771680c8e49b3db498a5da422db2adda8))
* Some non-blocking, callback-based C FFI ([#322](https://github.com/cdata/noosphere/issues/322)) ([693ce40](https://github.com/cdata/noosphere/commit/693ce40143acf99f758a12df2627e265ef105e03))
* Sphere sync and change diff in C FFI ([#210](https://github.com/cdata/noosphere/issues/210)) ([306d39c](https://github.com/cdata/noosphere/commit/306d39cdf6727fbeb34a49740b55f56834f4df07))
* Sphere traversal C FFI ([#292](https://github.com/cdata/noosphere/issues/292)) ([5d55e60](https://github.com/cdata/noosphere/commit/5d55e60789fcec6abdcc50df10f0038274972806))
* Sphere writes do not block immutable reads ([#321](https://github.com/cdata/noosphere/issues/321)) ([14373c5](https://github.com/cdata/noosphere/commit/14373c5281c091bb41623677571566a2788a7e3f))
* Syndicate sphere revisions to IPFS Kubo ([#177](https://github.com/cdata/noosphere/issues/177)) ([e269e04](https://github.com/cdata/noosphere/commit/e269e0484b73e0f5507406d57a2c06cf849bee3d))
* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284)) ([43bceaf](https://github.com/cdata/noosphere/commit/43bceafcc838c5b06565780f372bf7b401de288e))
* Update Cargo.toml to test action ([b21a3ce](https://github.com/cdata/noosphere/commit/b21a3ce7dc7760f63be8ea5efe51d19f819bfe30))
* Update IPLD-related dependencies ([#327](https://github.com/cdata/noosphere/issues/327)) ([5fdfadb](https://github.com/cdata/noosphere/commit/5fdfadb1656f9d6eef2dbbb8b00a598106bccf00))


### Bug Fixes

* FFI header generation: Use an ordered BTreeMap to replace class token keys so that class names that are subsets of other class names are replaced appropriately. ([#270](https://github.com/cdata/noosphere/issues/270)) ([4cf2e40](https://github.com/cdata/noosphere/commit/4cf2e4053c3caad3fc903d285c98b6ac459c9582))
* Reconfigure module dependencies so that noosphere-ipfs depends on noosphere-storage, and not the other way around creating a cycle. ([#254](https://github.com/cdata/noosphere/issues/254)) ([b79872a](https://github.com/cdata/noosphere/commit/b79872afd54c7b69d447dfe99e750bb6a813645c))
* Remove vestigial `tracing-core` dependency ([#348](https://github.com/cdata/noosphere/issues/348)) ([31528c6](https://github.com/cdata/noosphere/commit/31528c6083190b5298b90b9a8af7f4eff3836b99))
* Unreachable petname sequence is not an error ([#310](https://github.com/cdata/noosphere/issues/310)) ([96f2938](https://github.com/cdata/noosphere/commit/96f2938d76f41fe240466bc7cfe397f886aa7e04))


### Miscellaneous Chores

* Templatize the two IPFS HTTP APIs as noosphere_ipfs::IpfsClient, and reconfigure KuboStorage as IpfsStorage, operating on IpfsClient rather than a URL. ([#252](https://github.com/cdata/noosphere/issues/252)) ([518beae](https://github.com/cdata/noosphere/commit/518beae563bd04c921ee3c6641a7249f14c611e4))
* Update IPLD-adjacent dependencies ([#180](https://github.com/cdata/noosphere/issues/180)) ([1a1114b](https://github.com/cdata/noosphere/commit/1a1114b0c6277ea2c0d879e43191e962eb2e462b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * noosphere-core bumped from 0.10.2 to 0.11.0
    * noosphere-sphere bumped from 0.5.2 to 0.6.0
    * noosphere-storage bumped from 0.6.2 to 0.7.0
    * noosphere-api bumped from 0.7.7 to 0.8.0
    * noosphere-ipfs bumped from 0.4.2 to 0.5.0
    * noosphere-into bumped from 0.8.2 to 0.9.0
</details>

<details><summary>noosphere-api: 0.8.0</summary>

## [0.8.0](https://github.com/cdata/noosphere/compare/noosphere-api-v0.7.7...noosphere-api-v0.8.0) (2023-05-08)


###   BREAKING CHANGES

* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284))
* Fork `iroh-car` as `noosphere-car` ([#283](https://github.com/cdata/noosphere/issues/283))
* Petname resolution and synchronization in spheres and gateways ([#253](https://github.com/cdata/noosphere/issues/253))
* Several critical dependencies of this library were updated to new versions that contain breaking changes.
* The `StorageProvider` trait has been replaced by the `Storage` trait. This new trait allows for distinct backing implementations of `BlockStore` and `KeyValueStore`.
* The `.sphere` directory has a new layout; the files previously used to store metadata have been replaced with database metadata; the `blocks` directory is now called `storage`. At this time the easiest migration path is to initialize a new sphere and copy your existing files into it.
* Many APIs that previously asked for bare strings when a DID string was expected now expect a newtype called `Did` that wraps a string.
* The `noosphere-api` Client now holds an owned key instead of a reference.

### Features

* `SphereFs` is initialized with key material ([#140](https://github.com/cdata/noosphere/issues/140)) ([af48061](https://github.com/cdata/noosphere/commit/af4806114ca8f7703e0a888c7f369a4a4ed69c00))
* Add `noosphere` crate-based Swift package ([#131](https://github.com/cdata/noosphere/issues/131)) ([e1204c2](https://github.com/cdata/noosphere/commit/e1204c2a5822c3c0dbb7e61bbacffb2c1f49d8d8))
* Dot syntax when traversing by petname ([#306](https://github.com/cdata/noosphere/issues/306)) ([cd87b05](https://github.com/cdata/noosphere/commit/cd87b0533c21bbbd4d82332556e70ecc706a5531))
* Enable expired yet valid records in the name system. Update to ucan 0.2.0. ([#360](https://github.com/cdata/noosphere/issues/360)) ([3b0663a](https://github.com/cdata/noosphere/commit/3b0663abc7783a6d33dd47d20caae7597ab93ed0))
* Fork `iroh-car` as `noosphere-car` ([#283](https://github.com/cdata/noosphere/issues/283)) ([b0b7c38](https://github.com/cdata/noosphere/commit/b0b7c3835ff1ef271bbe0f833f6f7856fcc30de1))
* Petname resolution and synchronization in spheres and gateways ([#253](https://github.com/cdata/noosphere/issues/253)) ([f7ddfa7](https://github.com/cdata/noosphere/commit/f7ddfa7b65129efe795c6e3fca58cdc22799127a))
* Re-implement `noosphere-cli` in terms of `noosphere` ([#162](https://github.com/cdata/noosphere/issues/162)) ([1e83bbb](https://github.com/cdata/noosphere/commit/1e83bbb689642b878f4f6909d7dd4a6df56b29f9))
* Refactor storage interfaces ([#178](https://github.com/cdata/noosphere/issues/178)) ([4db55c4](https://github.com/cdata/noosphere/commit/4db55c4cba56b329a638a4227e7f3247ad8d319c))
* Sphere writes do not block immutable reads ([#321](https://github.com/cdata/noosphere/issues/321)) ([14373c5](https://github.com/cdata/noosphere/commit/14373c5281c091bb41623677571566a2788a7e3f))
* Syndicate sphere revisions to IPFS Kubo ([#177](https://github.com/cdata/noosphere/issues/177)) ([e269e04](https://github.com/cdata/noosphere/commit/e269e0484b73e0f5507406d57a2c06cf849bee3d))
* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284)) ([43bceaf](https://github.com/cdata/noosphere/commit/43bceafcc838c5b06565780f372bf7b401de288e))
* Update Cargo.toml to test action ([b21a3ce](https://github.com/cdata/noosphere/commit/b21a3ce7dc7760f63be8ea5efe51d19f819bfe30))
* Update IPLD-related dependencies ([#327](https://github.com/cdata/noosphere/issues/327)) ([5fdfadb](https://github.com/cdata/noosphere/commit/5fdfadb1656f9d6eef2dbbb8b00a598106bccf00))


### Bug Fixes

* **api:** Use rustls instead of OpenSSL ([1a0625a](https://github.com/cdata/noosphere/commit/1a0625ad79330d35ca137361297318bdbf29137e))
* Remove vestigial `tracing-core` dependency ([#348](https://github.com/cdata/noosphere/issues/348)) ([31528c6](https://github.com/cdata/noosphere/commit/31528c6083190b5298b90b9a8af7f4eff3836b99))
* Unreachable petname sequence is not an error ([#310](https://github.com/cdata/noosphere/issues/310)) ([96f2938](https://github.com/cdata/noosphere/commit/96f2938d76f41fe240466bc7cfe397f886aa7e04))


### Miscellaneous Chores

* Update IPLD-adjacent dependencies ([#180](https://github.com/cdata/noosphere/issues/180)) ([1a1114b](https://github.com/cdata/noosphere/commit/1a1114b0c6277ea2c0d879e43191e962eb2e462b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * noosphere-core bumped from 0.10.2 to 0.11.0
    * noosphere-storage bumped from 0.6.2 to 0.7.0
    * noosphere-car bumped from 0.1.1 to 0.2.0
</details>

<details><summary>noosphere-car: 0.2.0</summary>

## [0.2.0](https://github.com/cdata/noosphere/compare/noosphere-car-v0.1.1...noosphere-car-v0.2.0) (2023-05-08)


###   BREAKING CHANGES

* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284))
* Fork `iroh-car` as `noosphere-car` ([#283](https://github.com/cdata/noosphere/issues/283))

### Features

* Dot syntax when traversing by petname ([#306](https://github.com/cdata/noosphere/issues/306)) ([cd87b05](https://github.com/cdata/noosphere/commit/cd87b0533c21bbbd4d82332556e70ecc706a5531))
* Enable expired yet valid records in the name system. Update to ucan 0.2.0. ([#360](https://github.com/cdata/noosphere/issues/360)) ([3b0663a](https://github.com/cdata/noosphere/commit/3b0663abc7783a6d33dd47d20caae7597ab93ed0))
* Fork `iroh-car` as `noosphere-car` ([#283](https://github.com/cdata/noosphere/issues/283)) ([b0b7c38](https://github.com/cdata/noosphere/commit/b0b7c3835ff1ef271bbe0f833f6f7856fcc30de1))
* Sphere writes do not block immutable reads ([#321](https://github.com/cdata/noosphere/issues/321)) ([14373c5](https://github.com/cdata/noosphere/commit/14373c5281c091bb41623677571566a2788a7e3f))
* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284)) ([43bceaf](https://github.com/cdata/noosphere/commit/43bceafcc838c5b06565780f372bf7b401de288e))
* Update Cargo.toml to test action ([b21a3ce](https://github.com/cdata/noosphere/commit/b21a3ce7dc7760f63be8ea5efe51d19f819bfe30))
* Update IPLD-related dependencies ([#327](https://github.com/cdata/noosphere/issues/327)) ([5fdfadb](https://github.com/cdata/noosphere/commit/5fdfadb1656f9d6eef2dbbb8b00a598106bccf00))


### Bug Fixes

* Unreachable petname sequence is not an error ([#310](https://github.com/cdata/noosphere/issues/310)) ([96f2938](https://github.com/cdata/noosphere/commit/96f2938d76f41fe240466bc7cfe397f886aa7e04))
</details>

<details><summary>noosphere-cli: 0.11.0</summary>

## [0.11.0](https://github.com/cdata/noosphere/compare/noosphere-cli-v0.10.2...noosphere-cli-v0.11.0) (2023-05-08)


###   BREAKING CHANGES

* Revised tracing configuration ([#342](https://github.com/cdata/noosphere/issues/342))
* Apply breaking domain concept in anticipation of beta ([#298](https://github.com/cdata/noosphere/issues/298))
* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284))
* Fork `iroh-car` as `noosphere-car` ([#283](https://github.com/cdata/noosphere/issues/283))
* Revise links and gateway ([#278](https://github.com/cdata/noosphere/issues/278))
* Petname resolution and synchronization in spheres and gateways ([#253](https://github.com/cdata/noosphere/issues/253))
* `SphereFile` fields referring to a `revision` now refer to a `version` instead.
* Several critical dependencies of this library were updated to new versions that contain breaking changes.
* The `StorageProvider` trait has been replaced by the `Storage` trait. This new trait allows for distinct backing implementations of `BlockStore` and `KeyValueStore`.
* The `.sphere` directory has a new layout; the files previously used to store metadata have been replaced with database metadata; the `blocks` directory is now called `storage`. At this time the easiest migration path is to initialize a new sphere and copy your existing files into it.
* Many APIs that previously asked for bare strings when a DID string was expected now expect a newtype called `Did` that wraps a string.
* The `noosphere-api` Client now holds an owned key instead of a reference.

### Features

* `SphereFs` is initialized with key material ([#140](https://github.com/cdata/noosphere/issues/140)) ([af48061](https://github.com/cdata/noosphere/commit/af4806114ca8f7703e0a888c7f369a4a4ed69c00))
* Add `noosphere` crate-based Swift package ([#131](https://github.com/cdata/noosphere/issues/131)) ([e1204c2](https://github.com/cdata/noosphere/commit/e1204c2a5822c3c0dbb7e61bbacffb2c1f49d8d8))
* Beautify the Sphere Viewer demo app ([#186](https://github.com/cdata/noosphere/issues/186)) ([3e30fdb](https://github.com/cdata/noosphere/commit/3e30fdb5e2b6758397f05343491a36512a4f4a0c))
* **cli:** Find the nearest ancestor sphere ([#119](https://github.com/cdata/noosphere/issues/119)) ([9e33026](https://github.com/cdata/noosphere/commit/9e3302623db3af88df626ccb02ad8fa699e79223))
* Fork `iroh-car` as `noosphere-car` ([#283](https://github.com/cdata/noosphere/issues/283)) ([b0b7c38](https://github.com/cdata/noosphere/commit/b0b7c3835ff1ef271bbe0f833f6f7856fcc30de1))
* Improve "orb" error messaging ([#280](https://github.com/cdata/noosphere/issues/280)) ([d4b08f3](https://github.com/cdata/noosphere/commit/d4b08f3658f59b642395085483c0c79b0a03fb5d))
* Introduce `noosphere-gateway` crate ([#238](https://github.com/cdata/noosphere/issues/238)) ([791bc39](https://github.com/cdata/noosphere/commit/791bc3996cfac12cb077c3721f22d080a71d33ba))
* Introduce `noosphere-ipfs` crate ([#203](https://github.com/cdata/noosphere/issues/203)) ([ad1945b](https://github.com/cdata/noosphere/commit/ad1945bb7d64f169b6dac96807bf8d8e0c3ab482))
* Introduce web bindings and `orb` NPM package ([#182](https://github.com/cdata/noosphere/issues/182)) ([44170a2](https://github.com/cdata/noosphere/commit/44170a27be2e1d180b1cee153937ab2cef16a591))
* Petname resolution and synchronization in spheres and gateways ([#253](https://github.com/cdata/noosphere/issues/253)) ([f7ddfa7](https://github.com/cdata/noosphere/commit/f7ddfa7b65129efe795c6e3fca58cdc22799127a))
* Publish name record from gateway periodically. ([#334](https://github.com/cdata/noosphere/issues/334)) ([fc5e42f](https://github.com/cdata/noosphere/commit/fc5e42f2bd918fc1b3c448e55c611a99d49b00db))
* Re-implement `noosphere-cli` in terms of `noosphere` ([#162](https://github.com/cdata/noosphere/issues/162)) ([1e83bbb](https://github.com/cdata/noosphere/commit/1e83bbb689642b878f4f6909d7dd4a6df56b29f9))
* Refactor storage interfaces ([#178](https://github.com/cdata/noosphere/issues/178)) ([4db55c4](https://github.com/cdata/noosphere/commit/4db55c4cba56b329a638a4227e7f3247ad8d319c))
* Remove `Mutex` from NNS `ApiServer` for concurrency ([#357](https://github.com/cdata/noosphere/issues/357)) ([2347d10](https://github.com/cdata/noosphere/commit/2347d10490fbb7ecc219a3a09c1de21e11f66fa2))
* Revise links and gateway ([#278](https://github.com/cdata/noosphere/issues/278)) ([4cd2e3a](https://github.com/cdata/noosphere/commit/4cd2e3af8b10cdaae710d87e4b919b5180d10fec))
* Revised tracing configuration ([#342](https://github.com/cdata/noosphere/issues/342)) ([c4a4084](https://github.com/cdata/noosphere/commit/c4a4084771680c8e49b3db498a5da422db2adda8))
* Syndicate sphere revisions to IPFS Kubo ([#177](https://github.com/cdata/noosphere/issues/177)) ([e269e04](https://github.com/cdata/noosphere/commit/e269e0484b73e0f5507406d57a2c06cf849bee3d))
* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284)) ([43bceaf](https://github.com/cdata/noosphere/commit/43bceafcc838c5b06565780f372bf7b401de288e))
* Update IPLD-related dependencies ([#327](https://github.com/cdata/noosphere/issues/327)) ([5fdfadb](https://github.com/cdata/noosphere/commit/5fdfadb1656f9d6eef2dbbb8b00a598106bccf00))


### Bug Fixes

* Limit delegated UCAN's lifetime to authorization token's lifetime where appropriate. ([#249](https://github.com/cdata/noosphere/issues/249)) ([b62fb88](https://github.com/cdata/noosphere/commit/b62fb888e16718cb84f33aa93c14385ddef4d8d1))
* Recover from Kubo pin check ([#193](https://github.com/cdata/noosphere/issues/193)) ([b0e0851](https://github.com/cdata/noosphere/commit/b0e0851a5748c88c05977091abd780cf1a4f12ce))
* Unreachable petname sequence is not an error ([#310](https://github.com/cdata/noosphere/issues/310)) ([96f2938](https://github.com/cdata/noosphere/commit/96f2938d76f41fe240466bc7cfe397f886aa7e04))


### Miscellaneous Chores

* Apply breaking domain concept in anticipation of beta ([#298](https://github.com/cdata/noosphere/issues/298)) ([bd34ab4](https://github.com/cdata/noosphere/commit/bd34ab49b2d2c65cffe25657cf4d188d5c79d15f))
* Update IPLD-adjacent dependencies ([#180](https://github.com/cdata/noosphere/issues/180)) ([1a1114b](https://github.com/cdata/noosphere/commit/1a1114b0c6277ea2c0d879e43191e962eb2e462b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * noosphere-car bumped from 0.1.1 to 0.2.0
    * noosphere-ipfs bumped from 0.4.2 to 0.5.0
    * noosphere-core bumped from 0.10.2 to 0.11.0
    * noosphere-sphere bumped from 0.5.2 to 0.6.0
    * noosphere-storage bumped from 0.6.2 to 0.7.0
    * noosphere-api bumped from 0.7.7 to 0.8.0
    * noosphere-gateway bumped from 0.4.2 to 0.5.0
    * noosphere bumped from 0.10.2 to 0.11.0
  * dev-dependencies
    * noosphere-ns bumped from 0.6.2 to 0.7.0
</details>

<details><summary>noosphere-collections: 0.6.0</summary>

## [0.6.0](https://github.com/cdata/noosphere/compare/noosphere-collections-v0.5.2...noosphere-collections-v0.6.0) (2023-05-08)


###   BREAKING CHANGES

* Apply breaking domain concept in anticipation of beta ([#298](https://github.com/cdata/noosphere/issues/298))
* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284))
* `SphereFile` fields referring to a `revision` now refer to a `version` instead.
* Several critical dependencies of this library were updated to new versions that contain breaking changes.
* The `StorageProvider` trait has been replaced by the `Storage` trait. This new trait allows for distinct backing implementations of `BlockStore` and `KeyValueStore`.

### Features

* Always flush on SphereFS save ([#231](https://github.com/cdata/noosphere/issues/231)) ([bd151d5](https://github.com/cdata/noosphere/commit/bd151d5aca75b78b786d008177ab7d4e53e843bc))
* Beautify the Sphere Viewer demo app ([#186](https://github.com/cdata/noosphere/issues/186)) ([3e30fdb](https://github.com/cdata/noosphere/commit/3e30fdb5e2b6758397f05343491a36512a4f4a0c))
* Enable expired yet valid records in the name system. Update to ucan 0.2.0. ([#360](https://github.com/cdata/noosphere/issues/360)) ([3b0663a](https://github.com/cdata/noosphere/commit/3b0663abc7783a6d33dd47d20caae7597ab93ed0))
* Refactor storage interfaces ([#178](https://github.com/cdata/noosphere/issues/178)) ([4db55c4](https://github.com/cdata/noosphere/commit/4db55c4cba56b329a638a4227e7f3247ad8d319c))
* Sphere writes do not block immutable reads ([#321](https://github.com/cdata/noosphere/issues/321)) ([14373c5](https://github.com/cdata/noosphere/commit/14373c5281c091bb41623677571566a2788a7e3f))
* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284)) ([43bceaf](https://github.com/cdata/noosphere/commit/43bceafcc838c5b06565780f372bf7b401de288e))
* Update Cargo.toml to test action ([b21a3ce](https://github.com/cdata/noosphere/commit/b21a3ce7dc7760f63be8ea5efe51d19f819bfe30))
* Update IPLD-related dependencies ([#327](https://github.com/cdata/noosphere/issues/327)) ([5fdfadb](https://github.com/cdata/noosphere/commit/5fdfadb1656f9d6eef2dbbb8b00a598106bccf00))


### Miscellaneous Chores

* Apply breaking domain concept in anticipation of beta ([#298](https://github.com/cdata/noosphere/issues/298)) ([bd34ab4](https://github.com/cdata/noosphere/commit/bd34ab49b2d2c65cffe25657cf4d188d5c79d15f))
* Update IPLD-adjacent dependencies ([#180](https://github.com/cdata/noosphere/issues/180)) ([1a1114b](https://github.com/cdata/noosphere/commit/1a1114b0c6277ea2c0d879e43191e962eb2e462b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * noosphere-storage bumped from 0.6.2 to 0.7.0
</details>

<details><summary>noosphere-core: 0.11.0</summary>

## [0.11.0](https://github.com/cdata/noosphere/compare/noosphere-core-v0.10.2...noosphere-core-v0.11.0) (2023-05-08)


###   BREAKING CHANGES

* Revised tracing configuration ([#342](https://github.com/cdata/noosphere/issues/342))
* Apply breaking domain concept in anticipation of beta ([#298](https://github.com/cdata/noosphere/issues/298))
* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284))
* Fork `iroh-car` as `noosphere-car` ([#283](https://github.com/cdata/noosphere/issues/283))
* Revise links and gateway ([#278](https://github.com/cdata/noosphere/issues/278))
* Petname resolution and synchronization in spheres and gateways ([#253](https://github.com/cdata/noosphere/issues/253))
* Sphere sync and change diff in C FFI ([#210](https://github.com/cdata/noosphere/issues/210))
* `SphereFile` fields referring to a `revision` now refer to a `version` instead.
* Several critical dependencies of this library were updated to new versions that contain breaking changes.
* The `StorageProvider` trait has been replaced by the `Storage` trait. This new trait allows for distinct backing implementations of `BlockStore` and `KeyValueStore`.
* The `.sphere` directory has a new layout; the files previously used to store metadata have been replaced with database metadata; the `blocks` directory is now called `storage`. At this time the easiest migration path is to initialize a new sphere and copy your existing files into it.
* `SphereIpld` identity is now a `Did`
* Many APIs that previously asked for bare strings when a DID string was expected now expect a newtype called `Did` that wraps a string.
* The `noosphere-api` Client now holds an owned key instead of a reference.
* initial work on NameSystem, wrapping the underlying DHT network. ([#122](https://github.com/cdata/noosphere/issues/122))

### Features

* `SphereFs` is initialized with key material ([#140](https://github.com/cdata/noosphere/issues/140)) ([af48061](https://github.com/cdata/noosphere/commit/af4806114ca8f7703e0a888c7f369a4a4ed69c00))
* Add `noosphere` crate-based Swift package ([#131](https://github.com/cdata/noosphere/issues/131)) ([e1204c2](https://github.com/cdata/noosphere/commit/e1204c2a5822c3c0dbb7e61bbacffb2c1f49d8d8))
* Beautify the Sphere Viewer demo app ([#186](https://github.com/cdata/noosphere/issues/186)) ([3e30fdb](https://github.com/cdata/noosphere/commit/3e30fdb5e2b6758397f05343491a36512a4f4a0c))
* Dot syntax when traversing by petname ([#306](https://github.com/cdata/noosphere/issues/306)) ([cd87b05](https://github.com/cdata/noosphere/commit/cd87b0533c21bbbd4d82332556e70ecc706a5531))
* Enable expired yet valid records in the name system. Update to ucan 0.2.0. ([#360](https://github.com/cdata/noosphere/issues/360)) ([3b0663a](https://github.com/cdata/noosphere/commit/3b0663abc7783a6d33dd47d20caae7597ab93ed0))
* Fork `iroh-car` as `noosphere-car` ([#283](https://github.com/cdata/noosphere/issues/283)) ([b0b7c38](https://github.com/cdata/noosphere/commit/b0b7c3835ff1ef271bbe0f833f6f7856fcc30de1))
* General error handling in C FFI ([#219](https://github.com/cdata/noosphere/issues/219)) ([0a1952b](https://github.com/cdata/noosphere/commit/0a1952b34895071d2203505c95750d453bb110c6))
* Improvements to the NameSystem based on initial gateway integration ([#196](https://github.com/cdata/noosphere/issues/196)) ([4a6898e](https://github.com/cdata/noosphere/commit/4a6898e0aa8e1d96780226d384a6876eac122658))
* initial work on NameSystem, wrapping the underlying DHT network. ([#122](https://github.com/cdata/noosphere/issues/122)) ([656fb23](https://github.com/cdata/noosphere/commit/656fb23a5ce5a75b7f1de59444c1d866a9308d83))
* Introduce `Link`, a typed `Cid` ([#297](https://github.com/cdata/noosphere/issues/297)) ([9520826](https://github.com/cdata/noosphere/commit/9520826029235e5dc32adca77193b4f82b9de80c))
* Introduce `noosphere-gateway` crate ([#238](https://github.com/cdata/noosphere/issues/238)) ([791bc39](https://github.com/cdata/noosphere/commit/791bc3996cfac12cb077c3721f22d080a71d33ba))
* Introduce pet names to spheres ([#154](https://github.com/cdata/noosphere/issues/154)) ([7495796](https://github.com/cdata/noosphere/commit/74957968af7f7e51a6aa731192431fbf5e01215e))
* Introduce web bindings and `orb` NPM package ([#182](https://github.com/cdata/noosphere/issues/182)) ([44170a2](https://github.com/cdata/noosphere/commit/44170a27be2e1d180b1cee153937ab2cef16a591))
* Mutation and hydration for names ([#168](https://github.com/cdata/noosphere/issues/168)) ([5e2a1ca](https://github.com/cdata/noosphere/commit/5e2a1ca369875c425c0612c4ac7df0a942f8fcab))
* Petname resolution and synchronization in spheres and gateways ([#253](https://github.com/cdata/noosphere/issues/253)) ([f7ddfa7](https://github.com/cdata/noosphere/commit/f7ddfa7b65129efe795c6e3fca58cdc22799127a))
* Publish name record from gateway periodically. ([#334](https://github.com/cdata/noosphere/issues/334)) ([fc5e42f](https://github.com/cdata/noosphere/commit/fc5e42f2bd918fc1b3c448e55c611a99d49b00db))
* Re-implement `noosphere-cli` in terms of `noosphere` ([#162](https://github.com/cdata/noosphere/issues/162)) ([1e83bbb](https://github.com/cdata/noosphere/commit/1e83bbb689642b878f4f6909d7dd4a6df56b29f9))
* Refactor storage interfaces ([#178](https://github.com/cdata/noosphere/issues/178)) ([4db55c4](https://github.com/cdata/noosphere/commit/4db55c4cba56b329a638a4227e7f3247ad8d319c))
* Remove `Mutex` from NNS `ApiServer` for concurrency ([#357](https://github.com/cdata/noosphere/issues/357)) ([2347d10](https://github.com/cdata/noosphere/commit/2347d10490fbb7ecc219a3a09c1de21e11f66fa2))
* Revise links and gateway ([#278](https://github.com/cdata/noosphere/issues/278)) ([4cd2e3a](https://github.com/cdata/noosphere/commit/4cd2e3af8b10cdaae710d87e4b919b5180d10fec))
* Revised tracing configuration ([#342](https://github.com/cdata/noosphere/issues/342)) ([c4a4084](https://github.com/cdata/noosphere/commit/c4a4084771680c8e49b3db498a5da422db2adda8))
* Sphere sync and change diff in C FFI ([#210](https://github.com/cdata/noosphere/issues/210)) ([306d39c](https://github.com/cdata/noosphere/commit/306d39cdf6727fbeb34a49740b55f56834f4df07))
* Sphere writes do not block immutable reads ([#321](https://github.com/cdata/noosphere/issues/321)) ([14373c5](https://github.com/cdata/noosphere/commit/14373c5281c091bb41623677571566a2788a7e3f))
* Syndicate sphere revisions to IPFS Kubo ([#177](https://github.com/cdata/noosphere/issues/177)) ([e269e04](https://github.com/cdata/noosphere/commit/e269e0484b73e0f5507406d57a2c06cf849bee3d))
* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284)) ([43bceaf](https://github.com/cdata/noosphere/commit/43bceafcc838c5b06565780f372bf7b401de288e))
* Update Cargo.toml to test action ([b21a3ce](https://github.com/cdata/noosphere/commit/b21a3ce7dc7760f63be8ea5efe51d19f819bfe30))
* Update IPLD-related dependencies ([#327](https://github.com/cdata/noosphere/issues/327)) ([5fdfadb](https://github.com/cdata/noosphere/commit/5fdfadb1656f9d6eef2dbbb8b00a598106bccf00))


### Bug Fixes

* Ensure that sphere changes exclude `since` ([#216](https://github.com/cdata/noosphere/issues/216)) ([31fee07](https://github.com/cdata/noosphere/commit/31fee07424a019db21773947a5fe5a17a80f1c45))
* Limit delegated UCAN's lifetime to authorization token's lifetime where appropriate. ([#249](https://github.com/cdata/noosphere/issues/249)) ([b62fb88](https://github.com/cdata/noosphere/commit/b62fb888e16718cb84f33aa93c14385ddef4d8d1))
* Remove vestigial `tracing-core` dependency ([#348](https://github.com/cdata/noosphere/issues/348)) ([31528c6](https://github.com/cdata/noosphere/commit/31528c6083190b5298b90b9a8af7f4eff3836b99))


### Miscellaneous Chores

* Apply breaking domain concept in anticipation of beta ([#298](https://github.com/cdata/noosphere/issues/298)) ([bd34ab4](https://github.com/cdata/noosphere/commit/bd34ab49b2d2c65cffe25657cf4d188d5c79d15f))
* Update IPLD-adjacent dependencies ([#180](https://github.com/cdata/noosphere/issues/180)) ([1a1114b](https://github.com/cdata/noosphere/commit/1a1114b0c6277ea2c0d879e43191e962eb2e462b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * noosphere-storage bumped from 0.6.2 to 0.7.0
    * noosphere-collections bumped from 0.5.2 to 0.6.0
</details>

<details><summary>noosphere-gateway: 0.5.0</summary>

## [0.5.0](https://github.com/cdata/noosphere/compare/noosphere-gateway-v0.4.2...noosphere-gateway-v0.5.0) (2023-05-08)


###   BREAKING CHANGES

* Revised tracing configuration ([#342](https://github.com/cdata/noosphere/issues/342))
* Apply breaking domain concept in anticipation of beta ([#298](https://github.com/cdata/noosphere/issues/298))
* Sphere traversal C FFI ([#292](https://github.com/cdata/noosphere/issues/292))
* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284))
* Fork `iroh-car` as `noosphere-car` ([#283](https://github.com/cdata/noosphere/issues/283))
* Revise links and gateway ([#278](https://github.com/cdata/noosphere/issues/278))
* Petname resolution and synchronization in spheres and gateways ([#253](https://github.com/cdata/noosphere/issues/253))

### Features

* Enable expired yet valid records in the name system. Update to ucan 0.2.0. ([#360](https://github.com/cdata/noosphere/issues/360)) ([3b0663a](https://github.com/cdata/noosphere/commit/3b0663abc7783a6d33dd47d20caae7597ab93ed0))
* Enable support of quorum=0 for DHT during infra bootstrapping ([#335](https://github.com/cdata/noosphere/issues/335)) ([9d3619e](https://github.com/cdata/noosphere/commit/9d3619e0630a9fe3de867e08770df9d30682a91f))
* Fork `iroh-car` as `noosphere-car` ([#283](https://github.com/cdata/noosphere/issues/283)) ([b0b7c38](https://github.com/cdata/noosphere/commit/b0b7c3835ff1ef271bbe0f833f6f7856fcc30de1))
* Introduce `noosphere-gateway` crate ([#238](https://github.com/cdata/noosphere/issues/238)) ([791bc39](https://github.com/cdata/noosphere/commit/791bc3996cfac12cb077c3721f22d080a71d33ba))
* Petname resolution and synchronization in spheres and gateways ([#253](https://github.com/cdata/noosphere/issues/253)) ([f7ddfa7](https://github.com/cdata/noosphere/commit/f7ddfa7b65129efe795c6e3fca58cdc22799127a))
* Publish name record from gateway periodically. ([#334](https://github.com/cdata/noosphere/issues/334)) ([fc5e42f](https://github.com/cdata/noosphere/commit/fc5e42f2bd918fc1b3c448e55c611a99d49b00db))
* Remove `Mutex` from NNS `ApiServer` for concurrency ([#357](https://github.com/cdata/noosphere/issues/357)) ([2347d10](https://github.com/cdata/noosphere/commit/2347d10490fbb7ecc219a3a09c1de21e11f66fa2))
* Revise links and gateway ([#278](https://github.com/cdata/noosphere/issues/278)) ([4cd2e3a](https://github.com/cdata/noosphere/commit/4cd2e3af8b10cdaae710d87e4b919b5180d10fec))
* Revised tracing configuration ([#342](https://github.com/cdata/noosphere/issues/342)) ([c4a4084](https://github.com/cdata/noosphere/commit/c4a4084771680c8e49b3db498a5da422db2adda8))
* Sphere traversal C FFI ([#292](https://github.com/cdata/noosphere/issues/292)) ([5d55e60](https://github.com/cdata/noosphere/commit/5d55e60789fcec6abdcc50df10f0038274972806))
* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284)) ([43bceaf](https://github.com/cdata/noosphere/commit/43bceafcc838c5b06565780f372bf7b401de288e))
* Update IPLD-related dependencies ([#327](https://github.com/cdata/noosphere/issues/327)) ([5fdfadb](https://github.com/cdata/noosphere/commit/5fdfadb1656f9d6eef2dbbb8b00a598106bccf00))


### Bug Fixes

* Introduce `TryOrReset` to help worker threads ([#300](https://github.com/cdata/noosphere/issues/300)) ([5ea4b2c](https://github.com/cdata/noosphere/commit/5ea4b2c91d0b829e22f0c0b3cd22fe837eddf905))
* Use `wnfs-namefilter` instead of `wnfs` ([681d39a](https://github.com/cdata/noosphere/commit/681d39ab082227ab663053fd2170c2539b619ef0))


### Miscellaneous Chores

* Apply breaking domain concept in anticipation of beta ([#298](https://github.com/cdata/noosphere/issues/298)) ([bd34ab4](https://github.com/cdata/noosphere/commit/bd34ab49b2d2c65cffe25657cf4d188d5c79d15f))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * noosphere-car bumped from 0.1.1 to 0.2.0
    * noosphere-ipfs bumped from 0.4.2 to 0.5.0
    * noosphere-core bumped from 0.10.2 to 0.11.0
    * noosphere-ns bumped from 0.6.2 to 0.7.0
    * noosphere-storage bumped from 0.6.2 to 0.7.0
    * noosphere-sphere bumped from 0.5.2 to 0.6.0
    * noosphere-api bumped from 0.7.7 to 0.8.0
    * noosphere bumped from 0.10.2 to 0.11.0
</details>

<details><summary>noosphere-into: 0.9.0</summary>

## [0.9.0](https://github.com/cdata/noosphere/compare/noosphere-into-v0.8.2...noosphere-into-v0.9.0) (2023-05-08)


###   BREAKING CHANGES

* Revised tracing configuration ([#342](https://github.com/cdata/noosphere/issues/342))
* Apply breaking domain concept in anticipation of beta ([#298](https://github.com/cdata/noosphere/issues/298))
* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284))
* Fork `iroh-car` as `noosphere-car` ([#283](https://github.com/cdata/noosphere/issues/283))
* Revise links and gateway ([#278](https://github.com/cdata/noosphere/issues/278))
* Petname resolution and synchronization in spheres and gateways ([#253](https://github.com/cdata/noosphere/issues/253))
* `SphereFile` fields referring to a `revision` now refer to a `version` instead.
* Several critical dependencies of this library were updated to new versions that contain breaking changes.
* The `StorageProvider` trait has been replaced by the `Storage` trait. This new trait allows for distinct backing implementations of `BlockStore` and `KeyValueStore`.
* `SphereIpld` identity is now a `Did`
* Many APIs that previously asked for bare strings when a DID string was expected now expect a newtype called `Did` that wraps a string.

### Features

* `SphereFs` is initialized with key material ([#140](https://github.com/cdata/noosphere/issues/140)) ([af48061](https://github.com/cdata/noosphere/commit/af4806114ca8f7703e0a888c7f369a4a4ed69c00))
* Beautify the Sphere Viewer demo app ([#186](https://github.com/cdata/noosphere/issues/186)) ([3e30fdb](https://github.com/cdata/noosphere/commit/3e30fdb5e2b6758397f05343491a36512a4f4a0c))
* Dot syntax when traversing by petname ([#306](https://github.com/cdata/noosphere/issues/306)) ([cd87b05](https://github.com/cdata/noosphere/commit/cd87b0533c21bbbd4d82332556e70ecc706a5531))
* Enable expired yet valid records in the name system. Update to ucan 0.2.0. ([#360](https://github.com/cdata/noosphere/issues/360)) ([3b0663a](https://github.com/cdata/noosphere/commit/3b0663abc7783a6d33dd47d20caae7597ab93ed0))
* Fork `iroh-car` as `noosphere-car` ([#283](https://github.com/cdata/noosphere/issues/283)) ([b0b7c38](https://github.com/cdata/noosphere/commit/b0b7c3835ff1ef271bbe0f833f6f7856fcc30de1))
* Introduce pet names to spheres ([#154](https://github.com/cdata/noosphere/issues/154)) ([7495796](https://github.com/cdata/noosphere/commit/74957968af7f7e51a6aa731192431fbf5e01215e))
* Noosphere builds and runs tests on Windows ([#228](https://github.com/cdata/noosphere/issues/228)) ([d1320f0](https://github.com/cdata/noosphere/commit/d1320f08429c8f8090fd4612b56ebf9386414cc7))
* Petname resolution and synchronization in spheres and gateways ([#253](https://github.com/cdata/noosphere/issues/253)) ([f7ddfa7](https://github.com/cdata/noosphere/commit/f7ddfa7b65129efe795c6e3fca58cdc22799127a))
* Refactor storage interfaces ([#178](https://github.com/cdata/noosphere/issues/178)) ([4db55c4](https://github.com/cdata/noosphere/commit/4db55c4cba56b329a638a4227e7f3247ad8d319c))
* Revise links and gateway ([#278](https://github.com/cdata/noosphere/issues/278)) ([4cd2e3a](https://github.com/cdata/noosphere/commit/4cd2e3af8b10cdaae710d87e4b919b5180d10fec))
* Revised tracing configuration ([#342](https://github.com/cdata/noosphere/issues/342)) ([c4a4084](https://github.com/cdata/noosphere/commit/c4a4084771680c8e49b3db498a5da422db2adda8))
* Sphere writes do not block immutable reads ([#321](https://github.com/cdata/noosphere/issues/321)) ([14373c5](https://github.com/cdata/noosphere/commit/14373c5281c091bb41623677571566a2788a7e3f))
* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284)) ([43bceaf](https://github.com/cdata/noosphere/commit/43bceafcc838c5b06565780f372bf7b401de288e))
* Update Cargo.toml to test action ([b21a3ce](https://github.com/cdata/noosphere/commit/b21a3ce7dc7760f63be8ea5efe51d19f819bfe30))
* Update IPLD-related dependencies ([#327](https://github.com/cdata/noosphere/issues/327)) ([5fdfadb](https://github.com/cdata/noosphere/commit/5fdfadb1656f9d6eef2dbbb8b00a598106bccf00))


### Bug Fixes

* Remove vestigial `tracing-core` dependency ([#348](https://github.com/cdata/noosphere/issues/348)) ([31528c6](https://github.com/cdata/noosphere/commit/31528c6083190b5298b90b9a8af7f4eff3836b99))


### Miscellaneous Chores

* Apply breaking domain concept in anticipation of beta ([#298](https://github.com/cdata/noosphere/issues/298)) ([bd34ab4](https://github.com/cdata/noosphere/commit/bd34ab49b2d2c65cffe25657cf4d188d5c79d15f))
* Update IPLD-adjacent dependencies ([#180](https://github.com/cdata/noosphere/issues/180)) ([1a1114b](https://github.com/cdata/noosphere/commit/1a1114b0c6277ea2c0d879e43191e962eb2e462b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * noosphere-core bumped from 0.10.2 to 0.11.0
    * noosphere-storage bumped from 0.6.2 to 0.7.0
    * noosphere-sphere bumped from 0.5.2 to 0.6.0
  * dev-dependencies
    * noosphere-sphere bumped from 0.5.2 to 0.6.0
</details>

<details><summary>noosphere-ipfs: 0.5.0</summary>

## [0.5.0](https://github.com/cdata/noosphere/compare/noosphere-ipfs-v0.4.2...noosphere-ipfs-v0.5.0) (2023-05-08)


###   BREAKING CHANGES

* Revised tracing configuration ([#342](https://github.com/cdata/noosphere/issues/342))
* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284))
* Fork `iroh-car` as `noosphere-car` ([#283](https://github.com/cdata/noosphere/issues/283))
* Petname resolution and synchronization in spheres and gateways ([#253](https://github.com/cdata/noosphere/issues/253))
* Reconfigure module dependencies so that noosphere-ipfs depends on noosphere-storage, and not the other way around creating a cycle. ([#254](https://github.com/cdata/noosphere/issues/254))
* Templatize the two IPFS HTTP APIs as noosphere_ipfs::IpfsClient, and reconfigure KuboStorage as IpfsStorage, operating on IpfsClient rather than a URL. ([#252](https://github.com/cdata/noosphere/issues/252))

### Features

* Add instrumentation to `noosphere-ns` and `noosphere-ipfs`. ([#304](https://github.com/cdata/noosphere/issues/304)) ([3d6062d](https://github.com/cdata/noosphere/commit/3d6062d501e21393532b2db6f9ac740a041d91ba))
* Dot syntax when traversing by petname ([#306](https://github.com/cdata/noosphere/issues/306)) ([cd87b05](https://github.com/cdata/noosphere/commit/cd87b0533c21bbbd4d82332556e70ecc706a5531))
* Enable expired yet valid records in the name system. Update to ucan 0.2.0. ([#360](https://github.com/cdata/noosphere/issues/360)) ([3b0663a](https://github.com/cdata/noosphere/commit/3b0663abc7783a6d33dd47d20caae7597ab93ed0))
* Fork `iroh-car` as `noosphere-car` ([#283](https://github.com/cdata/noosphere/issues/283)) ([b0b7c38](https://github.com/cdata/noosphere/commit/b0b7c3835ff1ef271bbe0f833f6f7856fcc30de1))
* Implement `IpfsClient::get_block` for Kubo for orb/orb-ns integration with IPFS. ([#251](https://github.com/cdata/noosphere/issues/251)) ([f18db24](https://github.com/cdata/noosphere/commit/f18db2425d620165090afee9418d5f743a0cf579))
* Introduce `noosphere-gateway` crate ([#238](https://github.com/cdata/noosphere/issues/238)) ([791bc39](https://github.com/cdata/noosphere/commit/791bc3996cfac12cb077c3721f22d080a71d33ba))
* Introduce `noosphere-ipfs` crate ([#203](https://github.com/cdata/noosphere/issues/203)) ([ad1945b](https://github.com/cdata/noosphere/commit/ad1945bb7d64f169b6dac96807bf8d8e0c3ab482))
* Petname resolution and synchronization in spheres and gateways ([#253](https://github.com/cdata/noosphere/issues/253)) ([f7ddfa7](https://github.com/cdata/noosphere/commit/f7ddfa7b65129efe795c6e3fca58cdc22799127a))
* Revised tracing configuration ([#342](https://github.com/cdata/noosphere/issues/342)) ([c4a4084](https://github.com/cdata/noosphere/commit/c4a4084771680c8e49b3db498a5da422db2adda8))
* Sphere writes do not block immutable reads ([#321](https://github.com/cdata/noosphere/issues/321)) ([14373c5](https://github.com/cdata/noosphere/commit/14373c5281c091bb41623677571566a2788a7e3f))
* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284)) ([43bceaf](https://github.com/cdata/noosphere/commit/43bceafcc838c5b06565780f372bf7b401de288e))
* Update Cargo.toml to test action ([b21a3ce](https://github.com/cdata/noosphere/commit/b21a3ce7dc7760f63be8ea5efe51d19f819bfe30))
* Update IPLD-related dependencies ([#327](https://github.com/cdata/noosphere/issues/327)) ([5fdfadb](https://github.com/cdata/noosphere/commit/5fdfadb1656f9d6eef2dbbb8b00a598106bccf00))


### Bug Fixes

* Enable `noosphere-ipfs` to compile on its own ([764eeb7](https://github.com/cdata/noosphere/commit/764eeb7d24df2773afd5bce934f2de6fc2de2640))
* Introduce `TryOrReset` to help worker threads ([#300](https://github.com/cdata/noosphere/issues/300)) ([5ea4b2c](https://github.com/cdata/noosphere/commit/5ea4b2c91d0b829e22f0c0b3cd22fe837eddf905))
* Reconfigure module dependencies so that noosphere-ipfs depends on noosphere-storage, and not the other way around creating a cycle. ([#254](https://github.com/cdata/noosphere/issues/254)) ([b79872a](https://github.com/cdata/noosphere/commit/b79872afd54c7b69d447dfe99e750bb6a813645c))
* Remove vestigial `tracing-core` dependency ([#348](https://github.com/cdata/noosphere/issues/348)) ([31528c6](https://github.com/cdata/noosphere/commit/31528c6083190b5298b90b9a8af7f4eff3836b99))
* Several fixes for noosphere-ipfs as it gets further integrated ([#302](https://github.com/cdata/noosphere/issues/302)) ([9da4dd0](https://github.com/cdata/noosphere/commit/9da4dd063edf5bbf1a86556db64428d2ecb43f79))


### Miscellaneous Chores

* Templatize the two IPFS HTTP APIs as noosphere_ipfs::IpfsClient, and reconfigure KuboStorage as IpfsStorage, operating on IpfsClient rather than a URL. ([#252](https://github.com/cdata/noosphere/issues/252)) ([518beae](https://github.com/cdata/noosphere/commit/518beae563bd04c921ee3c6641a7249f14c611e4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * noosphere-storage bumped from 0.6.2 to 0.7.0
    * noosphere-car bumped from 0.1.1 to 0.2.0
  * dev-dependencies
    * noosphere-storage bumped from 0.6.2 to 0.7.0
    * noosphere-core bumped from 0.10.2 to 0.11.0
</details>

<details><summary>noosphere-ns: 0.7.0</summary>

## [0.7.0](https://github.com/cdata/noosphere/compare/noosphere-ns-v0.6.2...noosphere-ns-v0.7.0) (2023-05-08)


###   BREAKING CHANGES

* Revised tracing configuration ([#342](https://github.com/cdata/noosphere/issues/342))
* Petname resolution and synchronization in spheres and gateways ([#253](https://github.com/cdata/noosphere/issues/253))
* Templatize the two IPFS HTTP APIs as noosphere_ipfs::IpfsClient, and reconfigure KuboStorage as IpfsStorage, operating on IpfsClient rather than a URL. ([#252](https://github.com/cdata/noosphere/issues/252))
* upgrade libp2p to 0.50.0 ([#209](https://github.com/cdata/noosphere/issues/209))
* Several critical dependencies of this library were updated to new versions that contain breaking changes.
* The `StorageProvider` trait has been replaced by the `Storage` trait. This new trait allows for distinct backing implementations of `BlockStore` and `KeyValueStore`.
* The `.sphere` directory has a new layout; the files previously used to store metadata have been replaced with database metadata; the `blocks` directory is now called `storage`. At this time the easiest migration path is to initialize a new sphere and copy your existing files into it.
* initial work on NameSystem, wrapping the underlying DHT network. ([#122](https://github.com/cdata/noosphere/issues/122))

### Features

* Add instrumentation to `noosphere-ns` and `noosphere-ipfs`. ([#304](https://github.com/cdata/noosphere/issues/304)) ([3d6062d](https://github.com/cdata/noosphere/commit/3d6062d501e21393532b2db6f9ac740a041d91ba))
* cache 'peer_id' in orb-ns to provide a HTTP route that does not lock the NS mutex for testing. ([#303](https://github.com/cdata/noosphere/issues/303)) ([8e4769f](https://github.com/cdata/noosphere/commit/8e4769f548b486147a9b1e72d86555fe4246fa14))
* DHT configuration and status API ([#207](https://github.com/cdata/noosphere/issues/207)) ([7e671cf](https://github.com/cdata/noosphere/commit/7e671cfe06768e7faadd9d2573a11c899ae9cb22))
* Enable expired yet valid records in the name system. Update to ucan 0.2.0. ([#360](https://github.com/cdata/noosphere/issues/360)) ([3b0663a](https://github.com/cdata/noosphere/commit/3b0663abc7783a6d33dd47d20caae7597ab93ed0))
* Enable support of quorum=0 for DHT during infra bootstrapping ([#335](https://github.com/cdata/noosphere/issues/335)) ([9d3619e](https://github.com/cdata/noosphere/commit/9d3619e0630a9fe3de867e08770df9d30682a91f))
* Expose ipfs-api-url to orb-ns to integrate IPFS cid resolution in NS validation. ([#265](https://github.com/cdata/noosphere/issues/265)) ([d1bdc29](https://github.com/cdata/noosphere/commit/d1bdc29d28dc28e99eca794c11b4d190b7128dfe))
* Expose replication/publication/ttl intervals to NameSystemBuilder ([#130](https://github.com/cdata/noosphere/issues/130)) ([e20680e](https://github.com/cdata/noosphere/commit/e20680e225d53d8c658a9c6c2ba5dcb80d2a314e))
* Follow up of initial orb-ns implementation. ([#222](https://github.com/cdata/noosphere/issues/222)) ([bb4c53f](https://github.com/cdata/noosphere/commit/bb4c53f3e79de6f5f66cc5b83ec815864f6bc5ab))
* Implement a RecordValidator trait for the NameSystem DHT ([#129](https://github.com/cdata/noosphere/issues/129)) ([ba5560c](https://github.com/cdata/noosphere/commit/ba5560c031f2251a984eeaa0e0a7c95ad63e3c70))
* Improvements to the NameSystem based on initial gateway integration ([#196](https://github.com/cdata/noosphere/issues/196)) ([4a6898e](https://github.com/cdata/noosphere/commit/4a6898e0aa8e1d96780226d384a6876eac122658))
* initial work on NameSystem, wrapping the underlying DHT network. ([#122](https://github.com/cdata/noosphere/issues/122)) ([656fb23](https://github.com/cdata/noosphere/commit/656fb23a5ce5a75b7f1de59444c1d866a9308d83))
* Integration of orb-ns CLI into the Name System's operator API ([#218](https://github.com/cdata/noosphere/issues/218)) ([7f83fad](https://github.com/cdata/noosphere/commit/7f83fad1f318ec45eb47de76ca855f9eab4fe688))
* Introduce a `bootstrap` CLI in `noosphere-ns` to spin up DHT ([#143](https://github.com/cdata/noosphere/issues/143)) ([c5f2710](https://github.com/cdata/noosphere/commit/c5f27103cf6b8f597da0a3707fed45a494023920))
* Petname resolution and synchronization in spheres and gateways ([#253](https://github.com/cdata/noosphere/issues/253)) ([f7ddfa7](https://github.com/cdata/noosphere/commit/f7ddfa7b65129efe795c6e3fca58cdc22799127a))
* Re-implement `noosphere-cli` in terms of `noosphere` ([#162](https://github.com/cdata/noosphere/issues/162)) ([1e83bbb](https://github.com/cdata/noosphere/commit/1e83bbb689642b878f4f6909d7dd4a6df56b29f9))
* Refactor storage interfaces ([#178](https://github.com/cdata/noosphere/issues/178)) ([4db55c4](https://github.com/cdata/noosphere/commit/4db55c4cba56b329a638a4227e7f3247ad8d319c))
* Remove `Mutex` from NNS `ApiServer` for concurrency ([#357](https://github.com/cdata/noosphere/issues/357)) ([2347d10](https://github.com/cdata/noosphere/commit/2347d10490fbb7ecc219a3a09c1de21e11f66fa2))
* Revised tracing configuration ([#342](https://github.com/cdata/noosphere/issues/342)) ([c4a4084](https://github.com/cdata/noosphere/commit/c4a4084771680c8e49b3db498a5da422db2adda8))
* Update IPLD-related dependencies ([#327](https://github.com/cdata/noosphere/issues/327)) ([5fdfadb](https://github.com/cdata/noosphere/commit/5fdfadb1656f9d6eef2dbbb8b00a598106bccf00))
* upgrade libp2p to 0.50.0 ([#209](https://github.com/cdata/noosphere/issues/209)) ([14ab195](https://github.com/cdata/noosphere/commit/14ab195b797bcb23d1ed25a8eacc3fc37e30c0ce))
* Wrap validation storage with BlockRetryStore in orb-ns. ([#333](https://github.com/cdata/noosphere/issues/333)) ([8e7f287](https://github.com/cdata/noosphere/commit/8e7f287fe5cb6fe8842b9236ead57cccbdb8c90b))


### Bug Fixes

* Increase timeout in DHT network tests to satisfy CI, fixes [#311](https://github.com/cdata/noosphere/issues/311) ([#312](https://github.com/cdata/noosphere/issues/312)) ([2f9f1a6](https://github.com/cdata/noosphere/commit/2f9f1a6bbcc394672dfd2b93e4b1255f0fa9529b))
* Intermittent timeouts in DhtNode tests introduced in [#308](https://github.com/cdata/noosphere/issues/308) ([#316](https://github.com/cdata/noosphere/issues/316)) ([704652b](https://github.com/cdata/noosphere/commit/704652bba2a2d9b241799b97808c7a249f0c38a9))
* Limit delegated UCAN's lifetime to authorization token's lifetime where appropriate. ([#249](https://github.com/cdata/noosphere/issues/249)) ([b62fb88](https://github.com/cdata/noosphere/commit/b62fb888e16718cb84f33aa93c14385ddef4d8d1))


### Miscellaneous Chores

* Templatize the two IPFS HTTP APIs as noosphere_ipfs::IpfsClient, and reconfigure KuboStorage as IpfsStorage, operating on IpfsClient rather than a URL. ([#252](https://github.com/cdata/noosphere/issues/252)) ([518beae](https://github.com/cdata/noosphere/commit/518beae563bd04c921ee3c6641a7249f14c611e4))
* Update IPLD-adjacent dependencies ([#180](https://github.com/cdata/noosphere/issues/180)) ([1a1114b](https://github.com/cdata/noosphere/commit/1a1114b0c6277ea2c0d879e43191e962eb2e462b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * noosphere-storage bumped from 0.6.2 to 0.7.0
    * noosphere-core bumped from 0.10.2 to 0.11.0
    * noosphere bumped from 0.10.2 to 0.11.0
    * noosphere-ipfs bumped from 0.4.2 to 0.5.0
</details>

<details><summary>noosphere-sphere: 0.6.0</summary>

## [0.6.0](https://github.com/cdata/noosphere/compare/noosphere-sphere-v0.5.2...noosphere-sphere-v0.6.0) (2023-05-08)


###   BREAKING CHANGES

* Revised tracing configuration ([#342](https://github.com/cdata/noosphere/issues/342))
* Some non-blocking, callback-based C FFI ([#322](https://github.com/cdata/noosphere/issues/322))
* Apply breaking domain concept in anticipation of beta ([#298](https://github.com/cdata/noosphere/issues/298))
* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284))
* Fork `iroh-car` as `noosphere-car` ([#283](https://github.com/cdata/noosphere/issues/283))
* Revise links and gateway ([#278](https://github.com/cdata/noosphere/issues/278))
* Implement C FFI for petname management ([#271](https://github.com/cdata/noosphere/issues/271))
* Petname resolution and synchronization in spheres and gateways ([#253](https://github.com/cdata/noosphere/issues/253))

### Features

* Dot syntax when traversing by petname ([#306](https://github.com/cdata/noosphere/issues/306)) ([cd87b05](https://github.com/cdata/noosphere/commit/cd87b0533c21bbbd4d82332556e70ecc706a5531))
* Enable expired yet valid records in the name system. Update to ucan 0.2.0. ([#360](https://github.com/cdata/noosphere/issues/360)) ([3b0663a](https://github.com/cdata/noosphere/commit/3b0663abc7783a6d33dd47d20caae7597ab93ed0))
* Fork `iroh-car` as `noosphere-car` ([#283](https://github.com/cdata/noosphere/issues/283)) ([b0b7c38](https://github.com/cdata/noosphere/commit/b0b7c3835ff1ef271bbe0f833f6f7856fcc30de1))
* Implement C FFI for petname management ([#271](https://github.com/cdata/noosphere/issues/271)) ([d43c628](https://github.com/cdata/noosphere/commit/d43c6283c6b2374de503d70bd46c8df7d0337c3a))
* Petname resolution and synchronization in spheres and gateways ([#253](https://github.com/cdata/noosphere/issues/253)) ([f7ddfa7](https://github.com/cdata/noosphere/commit/f7ddfa7b65129efe795c6e3fca58cdc22799127a))
* Revise links and gateway ([#278](https://github.com/cdata/noosphere/issues/278)) ([4cd2e3a](https://github.com/cdata/noosphere/commit/4cd2e3af8b10cdaae710d87e4b919b5180d10fec))
* Revised tracing configuration ([#342](https://github.com/cdata/noosphere/issues/342)) ([c4a4084](https://github.com/cdata/noosphere/commit/c4a4084771680c8e49b3db498a5da422db2adda8))
* Some non-blocking, callback-based C FFI ([#322](https://github.com/cdata/noosphere/issues/322)) ([693ce40](https://github.com/cdata/noosphere/commit/693ce40143acf99f758a12df2627e265ef105e03))
* Sphere writes do not block immutable reads ([#321](https://github.com/cdata/noosphere/issues/321)) ([14373c5](https://github.com/cdata/noosphere/commit/14373c5281c091bb41623677571566a2788a7e3f))
* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284)) ([43bceaf](https://github.com/cdata/noosphere/commit/43bceafcc838c5b06565780f372bf7b401de288e))
* Update Cargo.toml to test action ([b21a3ce](https://github.com/cdata/noosphere/commit/b21a3ce7dc7760f63be8ea5efe51d19f819bfe30))
* Update IPLD-related dependencies ([#327](https://github.com/cdata/noosphere/issues/327)) ([5fdfadb](https://github.com/cdata/noosphere/commit/5fdfadb1656f9d6eef2dbbb8b00a598106bccf00))


### Bug Fixes

* Remove vestigial `tracing-core` dependency ([#348](https://github.com/cdata/noosphere/issues/348)) ([31528c6](https://github.com/cdata/noosphere/commit/31528c6083190b5298b90b9a8af7f4eff3836b99))
* Unreachable petname sequence is not an error ([#310](https://github.com/cdata/noosphere/issues/310)) ([96f2938](https://github.com/cdata/noosphere/commit/96f2938d76f41fe240466bc7cfe397f886aa7e04))


### Miscellaneous Chores

* Apply breaking domain concept in anticipation of beta ([#298](https://github.com/cdata/noosphere/issues/298)) ([bd34ab4](https://github.com/cdata/noosphere/commit/bd34ab49b2d2c65cffe25657cf4d188d5c79d15f))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * noosphere-core bumped from 0.10.2 to 0.11.0
    * noosphere-storage bumped from 0.6.2 to 0.7.0
    * noosphere-api bumped from 0.7.7 to 0.8.0
    * noosphere-ipfs bumped from 0.4.2 to 0.5.0
    * noosphere-car bumped from 0.1.1 to 0.2.0
</details>

<details><summary>noosphere-storage: 0.7.0</summary>

## [0.7.0](https://github.com/cdata/noosphere/compare/noosphere-storage-v0.6.2...noosphere-storage-v0.7.0) (2023-05-08)


###   BREAKING CHANGES

* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284))
* Petname resolution and synchronization in spheres and gateways ([#253](https://github.com/cdata/noosphere/issues/253))
* Reconfigure module dependencies so that noosphere-ipfs depends on noosphere-storage, and not the other way around creating a cycle. ([#254](https://github.com/cdata/noosphere/issues/254))
* Templatize the two IPFS HTTP APIs as noosphere_ipfs::IpfsClient, and reconfigure KuboStorage as IpfsStorage, operating on IpfsClient rather than a URL. ([#252](https://github.com/cdata/noosphere/issues/252))
* `SphereFile` fields referring to a `revision` now refer to a `version` instead.
* Several critical dependencies of this library were updated to new versions that contain breaking changes.
* The `StorageProvider` trait has been replaced by the `Storage` trait. This new trait allows for distinct backing implementations of `BlockStore` and `KeyValueStore`.
* The `.sphere` directory has a new layout; the files previously used to store metadata have been replaced with database metadata; the `blocks` directory is now called `storage`. At this time the easiest migration path is to initialize a new sphere and copy your existing files into it.
* The `noosphere-api` Client now holds an owned key instead of a reference.

### Features

* Add `noosphere` crate-based Swift package ([#131](https://github.com/cdata/noosphere/issues/131)) ([e1204c2](https://github.com/cdata/noosphere/commit/e1204c2a5822c3c0dbb7e61bbacffb2c1f49d8d8))
* Always flush on SphereFS save ([#231](https://github.com/cdata/noosphere/issues/231)) ([bd151d5](https://github.com/cdata/noosphere/commit/bd151d5aca75b78b786d008177ab7d4e53e843bc))
* Beautify the Sphere Viewer demo app ([#186](https://github.com/cdata/noosphere/issues/186)) ([3e30fdb](https://github.com/cdata/noosphere/commit/3e30fdb5e2b6758397f05343491a36512a4f4a0c))
* Dot syntax when traversing by petname ([#306](https://github.com/cdata/noosphere/issues/306)) ([cd87b05](https://github.com/cdata/noosphere/commit/cd87b0533c21bbbd4d82332556e70ecc706a5531))
* Enable expired yet valid records in the name system. Update to ucan 0.2.0. ([#360](https://github.com/cdata/noosphere/issues/360)) ([3b0663a](https://github.com/cdata/noosphere/commit/3b0663abc7783a6d33dd47d20caae7597ab93ed0))
* Improvements to the NameSystem based on initial gateway integration ([#196](https://github.com/cdata/noosphere/issues/196)) ([4a6898e](https://github.com/cdata/noosphere/commit/4a6898e0aa8e1d96780226d384a6876eac122658))
* Introduce web bindings and `orb` NPM package ([#182](https://github.com/cdata/noosphere/issues/182)) ([44170a2](https://github.com/cdata/noosphere/commit/44170a27be2e1d180b1cee153937ab2cef16a591))
* Petname resolution and synchronization in spheres and gateways ([#253](https://github.com/cdata/noosphere/issues/253)) ([f7ddfa7](https://github.com/cdata/noosphere/commit/f7ddfa7b65129efe795c6e3fca58cdc22799127a))
* Re-implement `noosphere-cli` in terms of `noosphere` ([#162](https://github.com/cdata/noosphere/issues/162)) ([1e83bbb](https://github.com/cdata/noosphere/commit/1e83bbb689642b878f4f6909d7dd4a6df56b29f9))
* Refactor storage interfaces ([#178](https://github.com/cdata/noosphere/issues/178)) ([4db55c4](https://github.com/cdata/noosphere/commit/4db55c4cba56b329a638a4227e7f3247ad8d319c))
* Sphere writes do not block immutable reads ([#321](https://github.com/cdata/noosphere/issues/321)) ([14373c5](https://github.com/cdata/noosphere/commit/14373c5281c091bb41623677571566a2788a7e3f))
* Syndicate sphere revisions to IPFS Kubo ([#177](https://github.com/cdata/noosphere/issues/177)) ([e269e04](https://github.com/cdata/noosphere/commit/e269e0484b73e0f5507406d57a2c06cf849bee3d))
* Traverse the Noosphere vast ([#284](https://github.com/cdata/noosphere/issues/284)) ([43bceaf](https://github.com/cdata/noosphere/commit/43bceafcc838c5b06565780f372bf7b401de288e))
* Update Cargo.toml to test action ([b21a3ce](https://github.com/cdata/noosphere/commit/b21a3ce7dc7760f63be8ea5efe51d19f819bfe30))
* Update IPLD-related dependencies ([#327](https://github.com/cdata/noosphere/issues/327)) ([5fdfadb](https://github.com/cdata/noosphere/commit/5fdfadb1656f9d6eef2dbbb8b00a598106bccf00))


### Bug Fixes

* Reconfigure module dependencies so that noosphere-ipfs depends on noosphere-storage, and not the other way around creating a cycle. ([#254](https://github.com/cdata/noosphere/issues/254)) ([b79872a](https://github.com/cdata/noosphere/commit/b79872afd54c7b69d447dfe99e750bb6a813645c))
* Remove vestigial `tracing-core` dependency ([#348](https://github.com/cdata/noosphere/issues/348)) ([31528c6](https://github.com/cdata/noosphere/commit/31528c6083190b5298b90b9a8af7f4eff3836b99))


### Miscellaneous Chores

* Templatize the two IPFS HTTP APIs as noosphere_ipfs::IpfsClient, and reconfigure KuboStorage as IpfsStorage, operating on IpfsClient rather than a URL. ([#252](https://github.com/cdata/noosphere/issues/252)) ([518beae](https://github.com/cdata/noosphere/commit/518beae563bd04c921ee3c6641a7249f14c611e4))
* Update IPLD-adjacent dependencies ([#180](https://github.com/cdata/noosphere/issues/180)) ([1a1114b](https://github.com/cdata/noosphere/commit/1a1114b0c6277ea2c0d879e43191e962eb2e462b))
</details>

---
This PR was generated with [Release Please](https://github.com/googleapis/release-please). See [documentation](https://github.com/googleapis/release-please#release-please).