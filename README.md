# yubi

[![test](https://github.com/yukihir0/yubi/workflows/test/badge.svg)](https://github.com/yukihir0/yubi/actions?query=workflow%3Atest)

"yubi" is a tool for automate check status of services based on specfile description.

It is named after the pointing and calling method, aka "yubi-sashi-kakunin".

## how to use

1. create specfile (`spec.yml`)

```
% cp spec.yml.sample spec.yml
% <editor> spec.yml
```

2. run

```
% yubi spec.yml
```

## input / output

### specfile (ipunt)

specfile has array of operator.

#### example

```
---
- operator: operator1
  ...
- operator: operator2
  ...
```

#### operator

##### GKEClusterStatus

GKEClusterStatus operator check gke cluster status within expected status.

###### requirement

GKEClusterStatus operator authenticates gcp by GOOGLE_APPLICATION_CREDENTIALS.

Set GOOGLE_APPLICATION_CREDENTIALS environmen variables.

```
% export GOOGLE_APPLICATION_CREDENTIALS=<path/to/credential>
```

###### format

| key      | description                           | type              | value                                                                            |
| -------- | ------------------------------------- | ----------------- | -------------------------------------------------------------------------------- |
| operator | operator                              | constant          | GKEClusterStatus                                                                 |
| project  | gcp project                           | string            | gcp_project                                                                      |
| location | gke cluster location (region or zone) | string            | gcp_region / gcp_zone                                                            |
| cluster  | gke cluster                           | string            | gke_cluster                                                                      |
| status   | gke cluster status                    | array of constant | Unspecified / Provisioning / Running / Reconciling / Stopping / Error / Degraded |

##### GKENodePoolStatus

GKENodePoolStatus operator check gke node pool status within expected status.

###### requirement

GKENodePoolStatus operator authenticates gcp by GOOGLE_APPLICATION_CREDENTIALS.

Set GOOGLE_APPLICATION_CREDENTIALS environmen variables.

```
% export GOOGLE_APPLICATION_CREDENTIALS=<path/to/credential>
```

###### format

| key       | description                           | type              | value                                                                            |
| --------- | ------------------------------------- | ----------------- | -------------------------------------------------------------------------------- |
| operator  | operator                              | constant          | GKEClusterStatus                                                                 |
| project   | gcp project                           | string            | gcp_project                                                                      |
| location  | gke cluster location (region or zone) | string            | gcp_region / gcp_zone                                                            |
| cluster   | gke cluster                           | string            | gke_cluster                                                                      |
| node_pool | gke node pool                         | string            | gke_node_pool                                                                    |
| status    | gke cluster status                    | array of constant | Unspecified / Provisioning / Running / Reconciling / Stopping / Error / Degraded |

### report (output)

report has summary and detail.

detail has array of pair of spec and spec_result.

#### example

```
---
summary:
  total: 2
  success: 1
  failure: 1
  error: 0
detail:
  - spec:
      operator: operator1
      ...
    spec_result:
      code: success
      description: "success!!"
  - spec:
      operator: operator2
      ...
    spec_result:
      code: failure
      description: "failure!!"
```

#### summary

| key     | description           | type   | value |
| ------- | --------------------- | ------ | ----- |
| total   | count of spec         | number | -     |
| success | count of success spec | number | -     |
| failure | count of failure spec | number | -     |
| error   | count of error spec   | number | -     |

#### detail

##### spec

###### format

spec format same as operator.

##### spec_result

###### format

| key         | description             | type     | value                     |
| ----------- | ----------------------- | -------- | ------------------------- |
| code        | spce_result code        | constant | success / failure / error |
| description | spec_result description | string   | -                         |

## development

### prepare

```
% cp spec.yml.sample spec.yml
% <editor> spec.yml
% export GOOGLE_APPLICATION_CREDENTIALS=<path/to/credential>
```

### run for development

```
% make development
```

### run for debug

```
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

### test coverage

```
% make test_coverage
```

### build

```
% make build
```

### renovate

```
% export RENOVATE_TOKEN=<github personal access token>
% make renovate
```

### renovate dry-run

```
% export RENOVATE_TOKEN=<github personal access token>
% make renovate_dry_run
```
