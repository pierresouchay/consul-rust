# Changes

## <Unreleased>

## 0.5.0

* Switch to asynchronous Reqwest clients.
* Rustify variable names.
* Improved documentation library-wide.
* Sealed internal trait implementations to prevent re-implementation.
* Export all traits in crate root.
* Move payload types into `crate::payload`.
* Rewrite request impls
* Update workflows
* Rename to `consul_oxide` to avoid confusion with the existing `consul` crate.

## 0.4.2

* Added `Config::new_from_consul_host`(#57)
* Various code refactoring
* new CI using GitHub actions instead of Travis

## 0.4.1 (2021-04-27)

* reqwest to 0.11, use blocking api
* support for X-Consul-Token
* [BUGFIX] Fix incorrect return values for services() in catalog
