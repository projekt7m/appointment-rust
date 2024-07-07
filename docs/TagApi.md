# \TagApi

All URIs are relative to *https://book.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_tag_by_id**](TagApi.md#delete_tag_by_id) | **DELETE** /tags/{id} | Delete a tag by ID
[**get_tag_by_id**](TagApi.md#get_tag_by_id) | **GET** /tags/{id} | Get a single tag by its ID
[**get_tags**](TagApi.md#get_tags) | **GET** /tags | Get all tags
[**post_tag**](TagApi.md#post_tag) | **POST** /tags | Create a new tag
[**put_tag_by_id**](TagApi.md#put_tag_by_id) | **PUT** /tags/{id} | Updates an existing tag



## delete_tag_by_id

> delete_tag_by_id(id)
Delete a tag by ID

Delete a tag by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the tag | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tag_by_id

> crate::models::Tag get_tag_by_id(id)
Get a single tag by its ID

Get a single tag by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the tag | [required] |

### Return type

[**crate::models::Tag**](Tag.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tags

> crate::models::TagData get_tags()
Get all tags

Get all tags  A tag is a customer defined attribute, that can be placed on other objects.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TagData**](TagData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_tag

> crate::models::Tag post_tag(new_tag)
Create a new tag

Create a new tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_tag** | [**NewTag**](NewTag.md) | The tag to be created | [required] |

### Return type

[**crate::models::Tag**](Tag.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_tag_by_id

> crate::models::Tag put_tag_by_id(id, new_tag)
Updates an existing tag

Updates an existing tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the tag | [required] |
**new_tag** | [**NewTag**](NewTag.md) | The updated tag | [required] |

### Return type

[**crate::models::Tag**](Tag.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

