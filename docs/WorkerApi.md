# \WorkerApi

All URIs are relative to *https://book.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**workers_get**](WorkerApi.md#workers_get) | **GET** /workers | 
[**workers_id_delete**](WorkerApi.md#workers_id_delete) | **DELETE** /workers/{id} | 
[**workers_id_get**](WorkerApi.md#workers_id_get) | **GET** /workers/{id} | 
[**workers_id_put**](WorkerApi.md#workers_id_put) | **PUT** /workers/{id} | 
[**workers_post**](WorkerApi.md#workers_post) | **POST** /workers | 



## workers_get

> crate::models::WorkerData workers_get()


a worker is a person that offers to do a service

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WorkerData**](WorkerData.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workers_id_delete

> workers_id_delete(id)


deletes the given worker

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | worker id | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workers_id_get

> crate::models::Worker workers_id_get(id)


Returns the given worker

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | worker id | [required] |

### Return type

[**crate::models::Worker**](Worker.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workers_id_put

> crate::models::Worker workers_id_put(id, worker)


updates the given worker

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | worker id | [required] |
**worker** | [**Worker**](Worker.md) | the updated worker | [required] |

### Return type

[**crate::models::Worker**](Worker.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workers_post

> crate::models::Worker workers_post(new_worker)


create a new worker

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_worker** | [**NewWorker**](NewWorker.md) | the worker to be created | [required] |

### Return type

[**crate::models::Worker**](Worker.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

