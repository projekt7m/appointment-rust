# \TagApi

All URIs are relative to *https://book.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tags_get**](TagApi.md#tags_get) | **GET** /tags | 
[**tags_id_delete**](TagApi.md#tags_id_delete) | **DELETE** /tags/{id} | 
[**tags_id_get**](TagApi.md#tags_id_get) | **GET** /tags/{id} | 
[**tags_id_put**](TagApi.md#tags_id_put) | **PUT** /tags/{id} | 
[**tags_post**](TagApi.md#tags_post) | **POST** /tags | 



## tags_get

> crate::models::TagData tags_get()


a tag is a customer defined attribute, that can be placed on other objects

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TagData**](TagData.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tags_id_delete

> tags_id_delete(id)


deletes the given tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | tag id | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tags_id_get

> crate::models::Tag tags_id_get(id)


Returns the given tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | tag id | [required] |

### Return type

[**crate::models::Tag**](Tag.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tags_id_put

> crate::models::Tag tags_id_put(id, tag)


updates the given tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | tag id | [required] |
**tag** | [**Tag**](Tag.md) | the updated tag | [required] |

### Return type

[**crate::models::Tag**](Tag.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tags_post

> crate::models::Tag tags_post(new_tag)


create a new tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_tag** | [**NewTag**](NewTag.md) | the tag to be created | [required] |

### Return type

[**crate::models::Tag**](Tag.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

