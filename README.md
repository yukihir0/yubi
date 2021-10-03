# yubi

[![test](https://github.com/yukihir0/yubi/workflows/test/badge.svg)](https://github.com/yukihir0/yubi/actions?query=workflow%3Atest)

"yubi" is a tool for automate check status of services.

It is named after the pointing and calling method, aka "yubi-sashi-kakunin".

## development

### prepare

```
% cp spec.yml.sample spec.yml
% <edit spec.yml>
```

### run for development

```
% export GOOGLE_APPLICATION_CREDENTIALS=<path/to/credential>
% make development
```

### run for debug

```
% export GOOGLE_APPLICATION_CREDENTIALS=<path/to/credential>
% make debug
```

### test

```
% make test
```

### test watch

```
% make test_watch
```

### build

```
% make build
```
