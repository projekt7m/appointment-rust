# \WorkerApi

All URIs are relative to *https://book.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_worker_by_id**](WorkerApi.md#delete_worker_by_id) | **DELETE** /workers/{id} | Deletes a worker by its ID
[**get_worker_by_id**](WorkerApi.md#get_worker_by_id) | **GET** /workers/{id} | Get a single worker by its ID
[**get_workers**](WorkerApi.md#get_workers) | **GET** /workers | Get the list of all workers
[**post_worker**](WorkerApi.md#post_worker) | **POST** /workers | Create a new worker
[**put_worker_by_id**](WorkerApi.md#put_worker_by_id) | **PUT** /workers/{id} | Update an existing worker



## delete_worker_by_id

> delete_worker_by_id(id)
Deletes a worker by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of the worker | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_worker_by_id

> models::Worker get_worker_by_id(id)
Get a single worker by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of the worker | [required] |

### Return type

[**models::Worker**](Worker.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workers

> models::ListWrapperWorker get_workers()
Get the list of all workers

A worker is a person that offers to do a service

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListWrapperWorker**](ListWrapper_Worker.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_worker

> models::Worker post_worker(new_worker)
Create a new worker

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_worker** | [**NewWorker**](NewWorker.md) | The worker to be created | [required] |

### Return type

[**models::Worker**](Worker.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_worker_by_id

> models::Worker put_worker_by_id(id, new_worker)
Update an existing worker

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of the worker | [required] |
**new_worker** | [**NewWorker**](NewWorker.md) | The updated worker | [required] |

### Return type

[**models::Worker**](Worker.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

