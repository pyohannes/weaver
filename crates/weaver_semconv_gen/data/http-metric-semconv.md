<!--- Hugo front matter used to generate the website version of this page:
linkTitle: Metrics
--->

# Semantic Conventions for HTTP Metrics

**Status**: [Mixed][DocumentStatus]

The conventions described in this section are HTTP specific. When HTTP operations occur,
metric events about those operations will be generated and reported to provide insight into the
operations. By adding HTTP attributes to metric events it allows for finely tuned filtering.

**Disclaimer:** These are initial HTTP metric instruments and attributes but more may be added in the future.

<!-- toc -->

- [HTTP Server](#http-server)
  - [Metric: `http.server.request.duration`](#metric-httpserverrequestduration)
  - [Metric: `http.server.active_requests`](#metric-httpserveractive_requests)
  - [Metric: `http.server.request.body.size`](#metric-httpserverrequestbodysize)
  - [Metric: `http.server.response.body.size`](#metric-httpserverresponsebodysize)
- [HTTP Client](#http-client)
  - [Metric: `http.client.request.duration`](#metric-httpclientrequestduration)
  - [Metric: `http.client.request.body.size`](#metric-httpclientrequestbodysize)
  - [Metric: `http.client.response.body.size`](#metric-httpclientresponsebodysize)
  - [Metric: `http.client.open_connections`](#metric-httpclientopen_connections)
  - [Metric: `http.client.connection.duration`](#metric-httpclientconnectionduration)
  - [Metric: `http.client.active_requests`](#metric-httpclientactive_requests)

<!-- tocstop -->

> **Warning**
> Existing HTTP instrumentations that are using
> [v1.20.0 of this document](https://github.com/open-telemetry/opentelemetry-specification/blob/v1.20.0/specification/metrics/semantic_conventions/http-metrics.md)
> (or prior):
>
> * SHOULD NOT change the version of the HTTP or networking conventions that they emit
>   until the HTTP semantic conventions are marked stable (HTTP stabilization will
>   include stabilization of a core set of networking conventions which are also used
>   in HTTP instrumentations). Conventions include, but are not limited to, attributes,
>   metric and span names, and unit of measure.
> * SHOULD introduce an environment variable `OTEL_SEMCONV_STABILITY_OPT_IN`
>   in the existing major version which is a comma-separated list of values.
>   The only values defined so far are:
>   * `http` - emit the new, stable HTTP and networking conventions,
>     and stop emitting the old experimental HTTP and networking conventions
>     that the instrumentation emitted previously.
>   * `http/dup` - emit both the old and the stable HTTP and networking conventions,
>     allowing for a seamless transition.
>   * The default behavior (in the absence of one of these values) is to continue
>     emitting whatever version of the old experimental HTTP and networking conventions
>     the instrumentation was emitting previously.
>   * Note: `http/dup` has higher precedence than `http` in case both values are present
> * SHOULD maintain (security patching at a minimum) the existing major version
>   for at least six months after it starts emitting both sets of conventions.
> * SHOULD drop the environment variable in the next major version.

## HTTP Server

### Metric: `http.server.request.duration`

**Status**: [Stable][DocumentStatus]

This metric is required.

When this metric is reported alongside an HTTP server span, the metric value SHOULD be the same as the HTTP server span duration.

This metric SHOULD be specified with
[`ExplicitBucketBoundaries`](https://github.com/open-telemetry/opentelemetry-specification/tree/v1.26.0/specification/metrics/api.md#instrument-advisory-parameters)
of `[ 0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1, 2.5, 5, 7.5, 10 ]`.

<!-- semconv metric.http.server.request.duration(metric_table) -->
| Name     | Instrument Type | Unit (UCUM) | Description    | Stability |
| -------- | --------------- | ----------- | -------------- | --------- |
| `http.server.request.duration` | Histogram | `s` | Duration of HTTP server requests. | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
<!-- endsemconv -->

<!-- semconv metric.http.server.request.duration(full) -->
| Attribute  | Type | Description  | Examples  | [Requirement Level](https://opentelemetry.io/docs/specs/semconv/general/attribute-requirement-level/) | Stability |
|---|---|---|---|---|---|
| [`http.request.method`](../attributes-registry/http.md) | string | HTTP request method. [1] | `GET`; `POST`; `HEAD` | `Required` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`url.scheme`](../attributes-registry/url.md) | string | The [URI scheme](https://www.rfc-editor.org/rfc/rfc3986#section-3.1) component identifying the used protocol. [2] | `http`; `https` | `Required` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`error.type`](../attributes-registry/error.md) | string | Describes a class of error the operation ended with. [3] | `timeout`; `java.net.UnknownHostException`; `server_certificate_invalid`; `500` | `Conditionally Required` If request has ended with an error. | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`http.response.status_code`](../attributes-registry/http.md) | int | [HTTP response status code](https://tools.ietf.org/html/rfc7231#section-6). | `200` | `Conditionally Required` If and only if one was received/sent. | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`http.route`](../attributes-registry/http.md) | string | The matched route, that is, the path template in the format used by the respective server framework. [4] | `/users/:userID?`; `{controller}/{action}/{id?}` | `Conditionally Required` If and only if it's available | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`network.protocol.name`](../attributes-registry/network.md) | string | [OSI application layer](https://osi-model.com/application-layer/) or non-OSI equivalent. [5] | `http`; `spdy` | `Conditionally Required` [6] | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`network.protocol.version`](../attributes-registry/network.md) | string | Version of the protocol specified in `network.protocol.name`. [7] | `1.0`; `1.1`; `2`; `3` | `Recommended` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`server.address`](../attributes-registry/server.md) | string | Name of the local HTTP server that received the request. [8] | `example.com`; `10.1.2.80`; `/tmp/my.sock` | `Opt-In` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`server.port`](../attributes-registry/server.md) | int | Port of the local HTTP server that received the request. [9] | `80`; `8080`; `443` | `Opt-In` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |

**[1]:** HTTP request method value SHOULD be "known" to the instrumentation.
By default, this convention defines "known" methods as the ones listed in [RFC9110](https://www.rfc-editor.org/rfc/rfc9110.html#name-methods)
and the PATCH method defined in [RFC5789](https://www.rfc-editor.org/rfc/rfc5789.html).

If the HTTP request method is not known to instrumentation, it MUST set the `http.request.method` attribute to `_OTHER`.

If the HTTP instrumentation could end up converting valid HTTP request methods to `_OTHER`, then it MUST provide a way to override
the list of known HTTP methods. If this override is done via environment variable, then the environment variable MUST be named
OTEL_INSTRUMENTATION_HTTP_KNOWN_METHODS and support a comma-separated list of case-sensitive known HTTP methods
(this list MUST be a full override of the default known method, it is not a list of known methods in addition to the defaults).

HTTP method names are case-sensitive and `http.request.method` attribute value MUST match a known HTTP method name exactly.
Instrumentations for specific web frameworks that consider HTTP methods to be case insensitive, SHOULD populate a canonical equivalent.
Tracing instrumentations that do so, MUST also set `http.request.method_original` to the original value.

**[2]:** The scheme of the original client request, if known (e.g. from [Forwarded#proto](https://developer.mozilla.org/docs/Web/HTTP/Headers/Forwarded#proto), [X-Forwarded-Proto](https://developer.mozilla.org/docs/Web/HTTP/Headers/X-Forwarded-Proto), or a similar header). Otherwise, the scheme of the immediate peer request.

**[3]:** If the request fails with an error before response status code was sent or received,
`error.type` SHOULD be set to exception type (its fully-qualified class name, if applicable)
or a component-specific low cardinality error identifier.

If response status code was sent or received and status indicates an error according to [HTTP span status definition](/docs/http/http-spans.md),
`error.type` SHOULD be set to the status code number (represented as a string), an exception type (if thrown) or a component-specific error identifier.

The `error.type` value SHOULD be predictable and SHOULD have low cardinality.
Instrumentations SHOULD document the list of errors they report.

The cardinality of `error.type` within one instrumentation library SHOULD be low, but
telemetry consumers that aggregate data from multiple instrumentation libraries and applications
should be prepared for `error.type` to have high cardinality at query time, when no
additional filters are applied.

If the request has completed successfully, instrumentations SHOULD NOT set `error.type`.

**[4]:** MUST NOT be populated when this is not supported by the HTTP server framework as the route attribute should have low-cardinality and the URI path can NOT substitute it.
SHOULD include the [application root](/docs/http/http-spans.md#http-server-definitions) if there is one.

**[5]:** The value SHOULD be normalized to lowercase.

**[6]:** If not `http` and `network.protocol.version` is set.

**[7]:** `network.protocol.version` refers to the version of the protocol used and might be different from the protocol client's version. If the HTTP client has a version of `0.27.2`, but sends HTTP version `1.1`, this attribute should be set to `1.1`.

**[8]:** See [Setting `server.address` and `server.port` attributes](/docs/http/http-spans.md#setting-serveraddress-and-serverport-attributes).
> **Warning**
> Since this attribute is based on HTTP headers, opting in to it may allow an attacker
> to trigger cardinality limits, degrading the usefulness of the metric.

**[9]:** See [Setting `server.address` and `server.port` attributes](/docs/http/http-spans.md#setting-serveraddress-and-serverport-attributes).
> **Warning**
> Since this attribute is based on HTTP headers, opting in to it may allow an attacker
> to trigger cardinality limits, degrading the usefulness of the metric.

`http.request.method` has the following list of well-known values. If one of them applies, then the respective value MUST be used; otherwise, a custom value MAY be used.

| Value  | Description | Stability |
|---|---|---|
| `CONNECT` | CONNECT method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `DELETE` | DELETE method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `GET` | GET method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `HEAD` | HEAD method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `OPTIONS` | OPTIONS method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `PATCH` | PATCH method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `POST` | POST method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `PUT` | PUT method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `TRACE` | TRACE method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `_OTHER` | Any HTTP method that the instrumentation has no prior knowledge of. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |

`error.type` has the following list of well-known values. If one of them applies, then the respective value MUST be used; otherwise, a custom value MAY be used.

| Value  | Description | Stability |
|---|---|---|
| `_OTHER` | A fallback error value to be used when the instrumentation doesn't define a custom value. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
<!-- endsemconv -->

### Metric: `http.server.active_requests`

**Status**: [Experimental][DocumentStatus]

This metric is optional.

<!-- semconv metric.http.server.active_requests(metric_table) -->
| Name     | Instrument Type | Unit (UCUM) | Description    |
| -------- | --------------- | ----------- | -------------- |
| `http.server.active_requests` | UpDownCounter | `{request}` | Number of active HTTP server requests. |
<!-- endsemconv -->

<!-- semconv metric.http.server.active_requests(full) -->
| Attribute  | Type | Description  | Examples  | Requirement Level |
|---|---|---|---|---|
| [`http.request.method`](../attributes-registry/http.md) | string | HTTP request method. [1] | `GET`; `POST`; `HEAD` | Required |
| [`server.address`](../attributes-registry/server.md) | string | Name of the local HTTP server that received the request. [2] | `example.com`; `10.1.2.80`; `/tmp/my.sock` | Opt-In |
| [`server.port`](../attributes-registry/server.md) | int | Port of the local HTTP server that received the request. [3] | `80`; `8080`; `443` | Opt-In |
| [`url.scheme`](../attributes-registry/url.md) | string | The [URI scheme](https://www.rfc-editor.org/rfc/rfc3986#section-3.1) component identifying the used protocol. | `http`; `https` | Required |

**[1]:** HTTP request method value SHOULD be "known" to the instrumentation.
By default, this convention defines "known" methods as the ones listed in [RFC9110](https://www.rfc-editor.org/rfc/rfc9110.html#name-methods)
and the PATCH method defined in [RFC5789](https://www.rfc-editor.org/rfc/rfc5789.html).

If the HTTP request method is not known to instrumentation, it MUST set the `http.request.method` attribute to `_OTHER`.

If the HTTP instrumentation could end up converting valid HTTP request methods to `_OTHER`, then it MUST provide a way to override
the list of known HTTP methods. If this override is done via environment variable, then the environment variable MUST be named
OTEL_INSTRUMENTATION_HTTP_KNOWN_METHODS and support a comma-separated list of case-sensitive known HTTP methods
(this list MUST be a full override of the default known method, it is not a list of known methods in addition to the defaults).

HTTP method names are case-sensitive and `http.request.method` attribute value MUST match a known HTTP method name exactly.
Instrumentations for specific web frameworks that consider HTTP methods to be case insensitive, SHOULD populate a canonical equivalent.
Tracing instrumentations that do so, MUST also set `http.request.method_original` to the original value.

**[2]:** See [Setting `server.address` and `server.port` attributes](/docs/http/http-spans.md#setting-serveraddress-and-serverport-attributes).
> **Warning**
> Since this attribute is based on HTTP headers, opting in to it may allow an attacker
> to trigger cardinality limits, degrading the usefulness of the metric.

**[3]:** See [Setting `server.address` and `server.port` attributes](/docs/http/http-spans.md#setting-serveraddress-and-serverport-attributes).
> **Warning**
> Since this attribute is based on HTTP headers, opting in to it may allow an attacker
> to trigger cardinality limits, degrading the usefulness of the metric.

`http.request.method` has the following list of well-known values. If one of them applies, then the respective value MUST be used, otherwise a custom value MAY be used.

| Value  | Description |
|---|---|
| `CONNECT` | CONNECT method. |
| `DELETE` | DELETE method. |
| `GET` | GET method. |
| `HEAD` | HEAD method. |
| `OPTIONS` | OPTIONS method. |
| `PATCH` | PATCH method. |
| `POST` | POST method. |
| `PUT` | PUT method. |
| `TRACE` | TRACE method. |
| `_OTHER` | Any HTTP method that the instrumentation has no prior knowledge of. |
<!-- endsemconv -->

### Metric: `http.server.request.body.size`

**Status**: [Experimental][DocumentStatus]

This metric is optional.

<!-- semconv metric.http.server.request.body.size(metric_table) -->
| Name     | Instrument Type | Unit (UCUM) | Description    | Stability |
| -------- | --------------- | ----------- | -------------- | --------- |
| `http.server.request.body.size` | Histogram | `By` | Size of HTTP server request bodies. [1] | ![Experimental](https://img.shields.io/badge/-experimental-blue) |

**[1]:** The size of the request payload body in bytes. This is the number of bytes transferred excluding headers and is often, but not always, present as the [Content-Length](https://www.rfc-editor.org/rfc/rfc9110.html#field.content-length) header. For requests using transport encoding, this should be the compressed size.
<!-- endsemconv -->

<!-- semconv metric.http.server.request.body.size(full) -->
| Attribute  | Type | Description  | Examples  | [Requirement Level](https://opentelemetry.io/docs/specs/semconv/general/attribute-requirement-level/) | Stability |
|---|---|---|---|---|---|
| [`http.request.method`](../attributes-registry/http.md) | string | HTTP request method. [1] | `GET`; `POST`; `HEAD` | `Required` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`url.scheme`](../attributes-registry/url.md) | string | The [URI scheme](https://www.rfc-editor.org/rfc/rfc3986#section-3.1) component identifying the used protocol. [2] | `http`; `https` | `Required` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`error.type`](../attributes-registry/error.md) | string | Describes a class of error the operation ended with. [3] | `timeout`; `java.net.UnknownHostException`; `server_certificate_invalid`; `500` | `Conditionally Required` If request has ended with an error. | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`http.response.status_code`](../attributes-registry/http.md) | int | [HTTP response status code](https://tools.ietf.org/html/rfc7231#section-6). | `200` | `Conditionally Required` If and only if one was received/sent. | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`http.route`](../attributes-registry/http.md) | string | The matched route, that is, the path template in the format used by the respective server framework. [4] | `/users/:userID?`; `{controller}/{action}/{id?}` | `Conditionally Required` If and only if it's available | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`network.protocol.name`](../attributes-registry/network.md) | string | [OSI application layer](https://osi-model.com/application-layer/) or non-OSI equivalent. [5] | `http`; `spdy` | `Conditionally Required` [6] | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`network.protocol.version`](../attributes-registry/network.md) | string | Version of the protocol specified in `network.protocol.name`. [7] | `1.0`; `1.1`; `2`; `3` | `Recommended` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`server.address`](../attributes-registry/server.md) | string | Name of the local HTTP server that received the request. [8] | `example.com`; `10.1.2.80`; `/tmp/my.sock` | `Opt-In` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`server.port`](../attributes-registry/server.md) | int | Port of the local HTTP server that received the request. [9] | `80`; `8080`; `443` | `Opt-In` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |

**[1]:** HTTP request method value SHOULD be "known" to the instrumentation.
By default, this convention defines "known" methods as the ones listed in [RFC9110](https://www.rfc-editor.org/rfc/rfc9110.html#name-methods)
and the PATCH method defined in [RFC5789](https://www.rfc-editor.org/rfc/rfc5789.html).

If the HTTP request method is not known to instrumentation, it MUST set the `http.request.method` attribute to `_OTHER`.

If the HTTP instrumentation could end up converting valid HTTP request methods to `_OTHER`, then it MUST provide a way to override
the list of known HTTP methods. If this override is done via environment variable, then the environment variable MUST be named
OTEL_INSTRUMENTATION_HTTP_KNOWN_METHODS and support a comma-separated list of case-sensitive known HTTP methods
(this list MUST be a full override of the default known method, it is not a list of known methods in addition to the defaults).

HTTP method names are case-sensitive and `http.request.method` attribute value MUST match a known HTTP method name exactly.
Instrumentations for specific web frameworks that consider HTTP methods to be case insensitive, SHOULD populate a canonical equivalent.
Tracing instrumentations that do so, MUST also set `http.request.method_original` to the original value.

**[2]:** The scheme of the original client request, if known (e.g. from [Forwarded#proto](https://developer.mozilla.org/docs/Web/HTTP/Headers/Forwarded#proto), [X-Forwarded-Proto](https://developer.mozilla.org/docs/Web/HTTP/Headers/X-Forwarded-Proto), or a similar header). Otherwise, the scheme of the immediate peer request.

**[3]:** If the request fails with an error before response status code was sent or received,
`error.type` SHOULD be set to exception type (its fully-qualified class name, if applicable)
or a component-specific low cardinality error identifier.

If response status code was sent or received and status indicates an error according to [HTTP span status definition](/docs/http/http-spans.md),
`error.type` SHOULD be set to the status code number (represented as a string), an exception type (if thrown) or a component-specific error identifier.

The `error.type` value SHOULD be predictable and SHOULD have low cardinality.
Instrumentations SHOULD document the list of errors they report.

The cardinality of `error.type` within one instrumentation library SHOULD be low, but
telemetry consumers that aggregate data from multiple instrumentation libraries and applications
should be prepared for `error.type` to have high cardinality at query time, when no
additional filters are applied.

If the request has completed successfully, instrumentations SHOULD NOT set `error.type`.

**[4]:** MUST NOT be populated when this is not supported by the HTTP server framework as the route attribute should have low-cardinality and the URI path can NOT substitute it.
SHOULD include the [application root](/docs/http/http-spans.md#http-server-definitions) if there is one.

**[5]:** The value SHOULD be normalized to lowercase.

**[6]:** If not `http` and `network.protocol.version` is set.

**[7]:** `network.protocol.version` refers to the version of the protocol used and might be different from the protocol client's version. If the HTTP client has a version of `0.27.2`, but sends HTTP version `1.1`, this attribute should be set to `1.1`.

**[8]:** See [Setting `server.address` and `server.port` attributes](/docs/http/http-spans.md#setting-serveraddress-and-serverport-attributes).
> **Warning**
> Since this attribute is based on HTTP headers, opting in to it may allow an attacker
> to trigger cardinality limits, degrading the usefulness of the metric.

**[9]:** See [Setting `server.address` and `server.port` attributes](/docs/http/http-spans.md#setting-serveraddress-and-serverport-attributes).
> **Warning**
> Since this attribute is based on HTTP headers, opting in to it may allow an attacker
> to trigger cardinality limits, degrading the usefulness of the metric.

`http.request.method` has the following list of well-known values. If one of them applies, then the respective value MUST be used; otherwise, a custom value MAY be used.

| Value  | Description | Stability |
|---|---|---|
| `CONNECT` | CONNECT method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `DELETE` | DELETE method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `GET` | GET method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `HEAD` | HEAD method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `OPTIONS` | OPTIONS method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `PATCH` | PATCH method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `POST` | POST method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `PUT` | PUT method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `TRACE` | TRACE method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `_OTHER` | Any HTTP method that the instrumentation has no prior knowledge of. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |

`error.type` has the following list of well-known values. If one of them applies, then the respective value MUST be used; otherwise, a custom value MAY be used.

| Value  | Description | Stability |
|---|---|---|
| `_OTHER` | A fallback error value to be used when the instrumentation doesn't define a custom value. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
<!-- endsemconv -->

### Metric: `http.server.response.body.size`

**Status**: [Experimental][DocumentStatus]

This metric is optional.

<!-- semconv metric.http.server.response.body.size(metric_table) -->
| Name     | Instrument Type | Unit (UCUM) | Description    | Stability |
| -------- | --------------- | ----------- | -------------- | --------- |
| `http.server.response.body.size` | Histogram | `By` | Size of HTTP server response bodies. [1] | ![Experimental](https://img.shields.io/badge/-experimental-blue) |

**[1]:** The size of the response payload body in bytes. This is the number of bytes transferred excluding headers and is often, but not always, present as the [Content-Length](https://www.rfc-editor.org/rfc/rfc9110.html#field.content-length) header. For requests using transport encoding, this should be the compressed size.
<!-- endsemconv -->

<!-- semconv metric.http.server.response.body.size(full) -->
| Attribute  | Type | Description  | Examples  | [Requirement Level](https://opentelemetry.io/docs/specs/semconv/general/attribute-requirement-level/) | Stability |
|---|---|---|---|---|---|
| [`http.request.method`](../attributes-registry/http.md) | string | HTTP request method. [1] | `GET`; `POST`; `HEAD` | `Required` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`url.scheme`](../attributes-registry/url.md) | string | The [URI scheme](https://www.rfc-editor.org/rfc/rfc3986#section-3.1) component identifying the used protocol. [2] | `http`; `https` | `Required` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`error.type`](../attributes-registry/error.md) | string | Describes a class of error the operation ended with. [3] | `timeout`; `java.net.UnknownHostException`; `server_certificate_invalid`; `500` | `Conditionally Required` If request has ended with an error. | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`http.response.status_code`](../attributes-registry/http.md) | int | [HTTP response status code](https://tools.ietf.org/html/rfc7231#section-6). | `200` | `Conditionally Required` If and only if one was received/sent. | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`http.route`](../attributes-registry/http.md) | string | The matched route, that is, the path template in the format used by the respective server framework. [4] | `/users/:userID?`; `{controller}/{action}/{id?}` | `Conditionally Required` If and only if it's available | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`network.protocol.name`](../attributes-registry/network.md) | string | [OSI application layer](https://osi-model.com/application-layer/) or non-OSI equivalent. [5] | `http`; `spdy` | `Conditionally Required` [6] | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`network.protocol.version`](../attributes-registry/network.md) | string | Version of the protocol specified in `network.protocol.name`. [7] | `1.0`; `1.1`; `2`; `3` | `Recommended` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`server.address`](../attributes-registry/server.md) | string | Name of the local HTTP server that received the request. [8] | `example.com`; `10.1.2.80`; `/tmp/my.sock` | `Opt-In` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`server.port`](../attributes-registry/server.md) | int | Port of the local HTTP server that received the request. [9] | `80`; `8080`; `443` | `Opt-In` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |

**[1]:** HTTP request method value SHOULD be "known" to the instrumentation.
By default, this convention defines "known" methods as the ones listed in [RFC9110](https://www.rfc-editor.org/rfc/rfc9110.html#name-methods)
and the PATCH method defined in [RFC5789](https://www.rfc-editor.org/rfc/rfc5789.html).

If the HTTP request method is not known to instrumentation, it MUST set the `http.request.method` attribute to `_OTHER`.

If the HTTP instrumentation could end up converting valid HTTP request methods to `_OTHER`, then it MUST provide a way to override
the list of known HTTP methods. If this override is done via environment variable, then the environment variable MUST be named
OTEL_INSTRUMENTATION_HTTP_KNOWN_METHODS and support a comma-separated list of case-sensitive known HTTP methods
(this list MUST be a full override of the default known method, it is not a list of known methods in addition to the defaults).

HTTP method names are case-sensitive and `http.request.method` attribute value MUST match a known HTTP method name exactly.
Instrumentations for specific web frameworks that consider HTTP methods to be case insensitive, SHOULD populate a canonical equivalent.
Tracing instrumentations that do so, MUST also set `http.request.method_original` to the original value.

**[2]:** The scheme of the original client request, if known (e.g. from [Forwarded#proto](https://developer.mozilla.org/docs/Web/HTTP/Headers/Forwarded#proto), [X-Forwarded-Proto](https://developer.mozilla.org/docs/Web/HTTP/Headers/X-Forwarded-Proto), or a similar header). Otherwise, the scheme of the immediate peer request.

**[3]:** If the request fails with an error before response status code was sent or received,
`error.type` SHOULD be set to exception type (its fully-qualified class name, if applicable)
or a component-specific low cardinality error identifier.

If response status code was sent or received and status indicates an error according to [HTTP span status definition](/docs/http/http-spans.md),
`error.type` SHOULD be set to the status code number (represented as a string), an exception type (if thrown) or a component-specific error identifier.

The `error.type` value SHOULD be predictable and SHOULD have low cardinality.
Instrumentations SHOULD document the list of errors they report.

The cardinality of `error.type` within one instrumentation library SHOULD be low, but
telemetry consumers that aggregate data from multiple instrumentation libraries and applications
should be prepared for `error.type` to have high cardinality at query time, when no
additional filters are applied.

If the request has completed successfully, instrumentations SHOULD NOT set `error.type`.

**[4]:** MUST NOT be populated when this is not supported by the HTTP server framework as the route attribute should have low-cardinality and the URI path can NOT substitute it.
SHOULD include the [application root](/docs/http/http-spans.md#http-server-definitions) if there is one.

**[5]:** The value SHOULD be normalized to lowercase.

**[6]:** If not `http` and `network.protocol.version` is set.

**[7]:** `network.protocol.version` refers to the version of the protocol used and might be different from the protocol client's version. If the HTTP client has a version of `0.27.2`, but sends HTTP version `1.1`, this attribute should be set to `1.1`.

**[8]:** See [Setting `server.address` and `server.port` attributes](/docs/http/http-spans.md#setting-serveraddress-and-serverport-attributes).
> **Warning**
> Since this attribute is based on HTTP headers, opting in to it may allow an attacker
> to trigger cardinality limits, degrading the usefulness of the metric.

**[9]:** See [Setting `server.address` and `server.port` attributes](/docs/http/http-spans.md#setting-serveraddress-and-serverport-attributes).
> **Warning**
> Since this attribute is based on HTTP headers, opting in to it may allow an attacker
> to trigger cardinality limits, degrading the usefulness of the metric.

`http.request.method` has the following list of well-known values. If one of them applies, then the respective value MUST be used; otherwise, a custom value MAY be used.

| Value  | Description | Stability |
|---|---|---|
| `CONNECT` | CONNECT method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `DELETE` | DELETE method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `GET` | GET method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `HEAD` | HEAD method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `OPTIONS` | OPTIONS method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `PATCH` | PATCH method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `POST` | POST method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `PUT` | PUT method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `TRACE` | TRACE method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `_OTHER` | Any HTTP method that the instrumentation has no prior knowledge of. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |

`error.type` has the following list of well-known values. If one of them applies, then the respective value MUST be used; otherwise, a custom value MAY be used.

| Value  | Description | Stability |
|---|---|---|
| `_OTHER` | A fallback error value to be used when the instrumentation doesn't define a custom value. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
<!-- endsemconv -->

## HTTP Client

### Metric: `http.client.request.duration`

**Status**: [Stable][DocumentStatus]

This metric is required.

When this metric is reported alongside an HTTP client span, the metric value SHOULD be the same as the HTTP client span duration.

This metric SHOULD be specified with
[`ExplicitBucketBoundaries`](https://github.com/open-telemetry/opentelemetry-specification/tree/v1.26.0/specification/metrics/api.md#instrument-advisory-parameters)
of `[ 0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1, 2.5, 5, 7.5, 10 ]`.

<!-- semconv metric.http.client.request.duration(metric_table) -->
| Name     | Instrument Type | Unit (UCUM) | Description    | Stability |
| -------- | --------------- | ----------- | -------------- | --------- |
| `http.client.request.duration` | Histogram | `s` | Duration of HTTP client requests. | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
<!-- endsemconv -->

<!-- semconv metric.http.client.request.duration(full) -->
| Attribute  | Type | Description  | Examples  | [Requirement Level](https://opentelemetry.io/docs/specs/semconv/general/attribute-requirement-level/) | Stability |
|---|---|---|---|---|---|
| [`http.request.method`](../attributes-registry/http.md) | string | HTTP request method. [1] | `GET`; `POST`; `HEAD` | `Required` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`server.address`](../attributes-registry/server.md) | string | Host identifier of the ["URI origin"](https://www.rfc-editor.org/rfc/rfc9110.html#name-uri-origin) HTTP request is sent to. [2] | `example.com`; `10.1.2.80`; `/tmp/my.sock` | `Required` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`server.port`](../attributes-registry/server.md) | int | Port identifier of the ["URI origin"](https://www.rfc-editor.org/rfc/rfc9110.html#name-uri-origin) HTTP request is sent to. [3] | `80`; `8080`; `443` | `Required` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`error.type`](../attributes-registry/error.md) | string | Describes a class of error the operation ended with. [4] | `timeout`; `java.net.UnknownHostException`; `server_certificate_invalid`; `500` | `Conditionally Required` If request has ended with an error. | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`http.response.status_code`](../attributes-registry/http.md) | int | [HTTP response status code](https://tools.ietf.org/html/rfc7231#section-6). | `200` | `Conditionally Required` If and only if one was received/sent. | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`network.protocol.name`](../attributes-registry/network.md) | string | [OSI application layer](https://osi-model.com/application-layer/) or non-OSI equivalent. [5] | `http`; `spdy` | `Conditionally Required` [6] | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`network.protocol.version`](../attributes-registry/network.md) | string | Version of the protocol specified in `network.protocol.name`. [7] | `1.0`; `1.1`; `2`; `3` | `Recommended` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`url.scheme`](../attributes-registry/url.md) | string | The [URI scheme](https://www.rfc-editor.org/rfc/rfc3986#section-3.1) component identifying the used protocol. | `http`; `https` | `Opt-In` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |

**[1]:** HTTP request method value SHOULD be "known" to the instrumentation.
By default, this convention defines "known" methods as the ones listed in [RFC9110](https://www.rfc-editor.org/rfc/rfc9110.html#name-methods)
and the PATCH method defined in [RFC5789](https://www.rfc-editor.org/rfc/rfc5789.html).

If the HTTP request method is not known to instrumentation, it MUST set the `http.request.method` attribute to `_OTHER`.

If the HTTP instrumentation could end up converting valid HTTP request methods to `_OTHER`, then it MUST provide a way to override
the list of known HTTP methods. If this override is done via environment variable, then the environment variable MUST be named
OTEL_INSTRUMENTATION_HTTP_KNOWN_METHODS and support a comma-separated list of case-sensitive known HTTP methods
(this list MUST be a full override of the default known method, it is not a list of known methods in addition to the defaults).

HTTP method names are case-sensitive and `http.request.method` attribute value MUST match a known HTTP method name exactly.
Instrumentations for specific web frameworks that consider HTTP methods to be case insensitive, SHOULD populate a canonical equivalent.
Tracing instrumentations that do so, MUST also set `http.request.method_original` to the original value.

**[2]:** If an HTTP client request is explicitly made to an IP address, e.g. `http://x.x.x.x:8080`, then `server.address` SHOULD be the IP address `x.x.x.x`. A DNS lookup SHOULD NOT be used.

**[3]:** When observed from the client side, and when communicating through an intermediary, `server.port` SHOULD represent the server port behind any intermediaries, for example proxies, if it's available.

**[4]:** If the request fails with an error before response status code was sent or received,
`error.type` SHOULD be set to exception type (its fully-qualified class name, if applicable)
or a component-specific low cardinality error identifier.

If response status code was sent or received and status indicates an error according to [HTTP span status definition](/docs/http/http-spans.md),
`error.type` SHOULD be set to the status code number (represented as a string), an exception type (if thrown) or a component-specific error identifier.

The `error.type` value SHOULD be predictable and SHOULD have low cardinality.
Instrumentations SHOULD document the list of errors they report.

The cardinality of `error.type` within one instrumentation library SHOULD be low, but
telemetry consumers that aggregate data from multiple instrumentation libraries and applications
should be prepared for `error.type` to have high cardinality at query time, when no
additional filters are applied.

If the request has completed successfully, instrumentations SHOULD NOT set `error.type`.

**[5]:** The value SHOULD be normalized to lowercase.

**[6]:** If not `http` and `network.protocol.version` is set.

**[7]:** `network.protocol.version` refers to the version of the protocol used and might be different from the protocol client's version. If the HTTP client has a version of `0.27.2`, but sends HTTP version `1.1`, this attribute should be set to `1.1`.

`http.request.method` has the following list of well-known values. If one of them applies, then the respective value MUST be used; otherwise, a custom value MAY be used.

| Value  | Description | Stability |
|---|---|---|
| `CONNECT` | CONNECT method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `DELETE` | DELETE method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `GET` | GET method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `HEAD` | HEAD method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `OPTIONS` | OPTIONS method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `PATCH` | PATCH method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `POST` | POST method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `PUT` | PUT method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `TRACE` | TRACE method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `_OTHER` | Any HTTP method that the instrumentation has no prior knowledge of. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |

`error.type` has the following list of well-known values. If one of them applies, then the respective value MUST be used; otherwise, a custom value MAY be used.

| Value  | Description | Stability |
|---|---|---|
| `_OTHER` | A fallback error value to be used when the instrumentation doesn't define a custom value. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
<!-- endsemconv -->

### Metric: `http.client.request.body.size`

**Status**: [Experimental][DocumentStatus]

This metric is optional.

<!-- semconv metric.http.client.request.body.size(metric_table) -->
| Name     | Instrument Type | Unit (UCUM) | Description    | Stability |
| -------- | --------------- | ----------- | -------------- | --------- |
| `http.client.request.body.size` | Histogram | `By` | Size of HTTP client request bodies. [1] | ![Experimental](https://img.shields.io/badge/-experimental-blue) |

**[1]:** The size of the request payload body in bytes. This is the number of bytes transferred excluding headers and is often, but not always, present as the [Content-Length](https://www.rfc-editor.org/rfc/rfc9110.html#field.content-length) header. For requests using transport encoding, this should be the compressed size.
<!-- endsemconv -->

<!-- semconv metric.http.client.request.body.size(full) -->
| Attribute  | Type | Description  | Examples  | [Requirement Level](https://opentelemetry.io/docs/specs/semconv/general/attribute-requirement-level/) | Stability |
|---|---|---|---|---|---|
| [`http.request.method`](../attributes-registry/http.md) | string | HTTP request method. [1] | `GET`; `POST`; `HEAD` | `Required` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`server.address`](../attributes-registry/server.md) | string | Host identifier of the ["URI origin"](https://www.rfc-editor.org/rfc/rfc9110.html#name-uri-origin) HTTP request is sent to. [2] | `example.com`; `10.1.2.80`; `/tmp/my.sock` | `Required` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`server.port`](../attributes-registry/server.md) | int | Port identifier of the ["URI origin"](https://www.rfc-editor.org/rfc/rfc9110.html#name-uri-origin) HTTP request is sent to. [3] | `80`; `8080`; `443` | `Required` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`error.type`](../attributes-registry/error.md) | string | Describes a class of error the operation ended with. [4] | `timeout`; `java.net.UnknownHostException`; `server_certificate_invalid`; `500` | `Conditionally Required` If request has ended with an error. | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`http.response.status_code`](../attributes-registry/http.md) | int | [HTTP response status code](https://tools.ietf.org/html/rfc7231#section-6). | `200` | `Conditionally Required` If and only if one was received/sent. | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`network.protocol.name`](../attributes-registry/network.md) | string | [OSI application layer](https://osi-model.com/application-layer/) or non-OSI equivalent. [5] | `http`; `spdy` | `Conditionally Required` [6] | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`network.protocol.version`](../attributes-registry/network.md) | string | Version of the protocol specified in `network.protocol.name`. [7] | `1.0`; `1.1`; `2`; `3` | `Recommended` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`url.scheme`](../attributes-registry/url.md) | string | The [URI scheme](https://www.rfc-editor.org/rfc/rfc3986#section-3.1) component identifying the used protocol. | `http`; `https` | `Opt-In` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |

**[1]:** HTTP request method value SHOULD be "known" to the instrumentation.
By default, this convention defines "known" methods as the ones listed in [RFC9110](https://www.rfc-editor.org/rfc/rfc9110.html#name-methods)
and the PATCH method defined in [RFC5789](https://www.rfc-editor.org/rfc/rfc5789.html).

If the HTTP request method is not known to instrumentation, it MUST set the `http.request.method` attribute to `_OTHER`.

If the HTTP instrumentation could end up converting valid HTTP request methods to `_OTHER`, then it MUST provide a way to override
the list of known HTTP methods. If this override is done via environment variable, then the environment variable MUST be named
OTEL_INSTRUMENTATION_HTTP_KNOWN_METHODS and support a comma-separated list of case-sensitive known HTTP methods
(this list MUST be a full override of the default known method, it is not a list of known methods in addition to the defaults).

HTTP method names are case-sensitive and `http.request.method` attribute value MUST match a known HTTP method name exactly.
Instrumentations for specific web frameworks that consider HTTP methods to be case insensitive, SHOULD populate a canonical equivalent.
Tracing instrumentations that do so, MUST also set `http.request.method_original` to the original value.

**[2]:** If an HTTP client request is explicitly made to an IP address, e.g. `http://x.x.x.x:8080`, then `server.address` SHOULD be the IP address `x.x.x.x`. A DNS lookup SHOULD NOT be used.

**[3]:** When observed from the client side, and when communicating through an intermediary, `server.port` SHOULD represent the server port behind any intermediaries, for example proxies, if it's available.

**[4]:** If the request fails with an error before response status code was sent or received,
`error.type` SHOULD be set to exception type (its fully-qualified class name, if applicable)
or a component-specific low cardinality error identifier.

If response status code was sent or received and status indicates an error according to [HTTP span status definition](/docs/http/http-spans.md),
`error.type` SHOULD be set to the status code number (represented as a string), an exception type (if thrown) or a component-specific error identifier.

The `error.type` value SHOULD be predictable and SHOULD have low cardinality.
Instrumentations SHOULD document the list of errors they report.

The cardinality of `error.type` within one instrumentation library SHOULD be low, but
telemetry consumers that aggregate data from multiple instrumentation libraries and applications
should be prepared for `error.type` to have high cardinality at query time, when no
additional filters are applied.

If the request has completed successfully, instrumentations SHOULD NOT set `error.type`.

**[5]:** The value SHOULD be normalized to lowercase.

**[6]:** If not `http` and `network.protocol.version` is set.

**[7]:** `network.protocol.version` refers to the version of the protocol used and might be different from the protocol client's version. If the HTTP client has a version of `0.27.2`, but sends HTTP version `1.1`, this attribute should be set to `1.1`.

`http.request.method` has the following list of well-known values. If one of them applies, then the respective value MUST be used; otherwise, a custom value MAY be used.

| Value  | Description | Stability |
|---|---|---|
| `CONNECT` | CONNECT method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `DELETE` | DELETE method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `GET` | GET method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `HEAD` | HEAD method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `OPTIONS` | OPTIONS method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `PATCH` | PATCH method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `POST` | POST method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `PUT` | PUT method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `TRACE` | TRACE method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `_OTHER` | Any HTTP method that the instrumentation has no prior knowledge of. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |

`error.type` has the following list of well-known values. If one of them applies, then the respective value MUST be used; otherwise, a custom value MAY be used.

| Value  | Description | Stability |
|---|---|---|
| `_OTHER` | A fallback error value to be used when the instrumentation doesn't define a custom value. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
<!-- endsemconv -->

### Metric: `http.client.response.body.size`

**Status**: [Experimental][DocumentStatus]

This metric is optional.

<!-- semconv metric.http.client.response.body.size(metric_table) -->
| Name     | Instrument Type | Unit (UCUM) | Description    | Stability |
| -------- | --------------- | ----------- | -------------- | --------- |
| `http.client.response.body.size` | Histogram | `By` | Size of HTTP client response bodies. [1] | ![Experimental](https://img.shields.io/badge/-experimental-blue) |

**[1]:** The size of the response payload body in bytes. This is the number of bytes transferred excluding headers and is often, but not always, present as the [Content-Length](https://www.rfc-editor.org/rfc/rfc9110.html#field.content-length) header. For requests using transport encoding, this should be the compressed size.
<!-- endsemconv -->

<!-- semconv metric.http.client.response.body.size(full) -->
| Attribute  | Type | Description  | Examples  | [Requirement Level](https://opentelemetry.io/docs/specs/semconv/general/attribute-requirement-level/) | Stability |
|---|---|---|---|---|---|
| [`http.request.method`](../attributes-registry/http.md) | string | HTTP request method. [1] | `GET`; `POST`; `HEAD` | `Required` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`server.address`](../attributes-registry/server.md) | string | Host identifier of the ["URI origin"](https://www.rfc-editor.org/rfc/rfc9110.html#name-uri-origin) HTTP request is sent to. [2] | `example.com`; `10.1.2.80`; `/tmp/my.sock` | `Required` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`server.port`](../attributes-registry/server.md) | int | Port identifier of the ["URI origin"](https://www.rfc-editor.org/rfc/rfc9110.html#name-uri-origin) HTTP request is sent to. [3] | `80`; `8080`; `443` | `Required` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`error.type`](../attributes-registry/error.md) | string | Describes a class of error the operation ended with. [4] | `timeout`; `java.net.UnknownHostException`; `server_certificate_invalid`; `500` | `Conditionally Required` If request has ended with an error. | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`http.response.status_code`](../attributes-registry/http.md) | int | [HTTP response status code](https://tools.ietf.org/html/rfc7231#section-6). | `200` | `Conditionally Required` If and only if one was received/sent. | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`network.protocol.name`](../attributes-registry/network.md) | string | [OSI application layer](https://osi-model.com/application-layer/) or non-OSI equivalent. [5] | `http`; `spdy` | `Conditionally Required` [6] | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`network.protocol.version`](../attributes-registry/network.md) | string | Version of the protocol specified in `network.protocol.name`. [7] | `1.0`; `1.1`; `2`; `3` | `Recommended` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`url.scheme`](../attributes-registry/url.md) | string | The [URI scheme](https://www.rfc-editor.org/rfc/rfc3986#section-3.1) component identifying the used protocol. | `http`; `https` | `Opt-In` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |

**[1]:** HTTP request method value SHOULD be "known" to the instrumentation.
By default, this convention defines "known" methods as the ones listed in [RFC9110](https://www.rfc-editor.org/rfc/rfc9110.html#name-methods)
and the PATCH method defined in [RFC5789](https://www.rfc-editor.org/rfc/rfc5789.html).

If the HTTP request method is not known to instrumentation, it MUST set the `http.request.method` attribute to `_OTHER`.

If the HTTP instrumentation could end up converting valid HTTP request methods to `_OTHER`, then it MUST provide a way to override
the list of known HTTP methods. If this override is done via environment variable, then the environment variable MUST be named
OTEL_INSTRUMENTATION_HTTP_KNOWN_METHODS and support a comma-separated list of case-sensitive known HTTP methods
(this list MUST be a full override of the default known method, it is not a list of known methods in addition to the defaults).

HTTP method names are case-sensitive and `http.request.method` attribute value MUST match a known HTTP method name exactly.
Instrumentations for specific web frameworks that consider HTTP methods to be case insensitive, SHOULD populate a canonical equivalent.
Tracing instrumentations that do so, MUST also set `http.request.method_original` to the original value.

**[2]:** If an HTTP client request is explicitly made to an IP address, e.g. `http://x.x.x.x:8080`, then `server.address` SHOULD be the IP address `x.x.x.x`. A DNS lookup SHOULD NOT be used.

**[3]:** When observed from the client side, and when communicating through an intermediary, `server.port` SHOULD represent the server port behind any intermediaries, for example proxies, if it's available.

**[4]:** If the request fails with an error before response status code was sent or received,
`error.type` SHOULD be set to exception type (its fully-qualified class name, if applicable)
or a component-specific low cardinality error identifier.

If response status code was sent or received and status indicates an error according to [HTTP span status definition](/docs/http/http-spans.md),
`error.type` SHOULD be set to the status code number (represented as a string), an exception type (if thrown) or a component-specific error identifier.

The `error.type` value SHOULD be predictable and SHOULD have low cardinality.
Instrumentations SHOULD document the list of errors they report.

The cardinality of `error.type` within one instrumentation library SHOULD be low, but
telemetry consumers that aggregate data from multiple instrumentation libraries and applications
should be prepared for `error.type` to have high cardinality at query time, when no
additional filters are applied.

If the request has completed successfully, instrumentations SHOULD NOT set `error.type`.

**[5]:** The value SHOULD be normalized to lowercase.

**[6]:** If not `http` and `network.protocol.version` is set.

**[7]:** `network.protocol.version` refers to the version of the protocol used and might be different from the protocol client's version. If the HTTP client has a version of `0.27.2`, but sends HTTP version `1.1`, this attribute should be set to `1.1`.

`http.request.method` has the following list of well-known values. If one of them applies, then the respective value MUST be used; otherwise, a custom value MAY be used.

| Value  | Description | Stability |
|---|---|---|
| `CONNECT` | CONNECT method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `DELETE` | DELETE method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `GET` | GET method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `HEAD` | HEAD method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `OPTIONS` | OPTIONS method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `PATCH` | PATCH method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `POST` | POST method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `PUT` | PUT method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `TRACE` | TRACE method. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
| `_OTHER` | Any HTTP method that the instrumentation has no prior knowledge of. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |

`error.type` has the following list of well-known values. If one of them applies, then the respective value MUST be used; otherwise, a custom value MAY be used.

| Value  | Description | Stability |
|---|---|---|
| `_OTHER` | A fallback error value to be used when the instrumentation doesn't define a custom value. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
<!-- endsemconv -->

### Metric: `http.client.open_connections`

**Status**: [Experimental][DocumentStatus]

This metric is optional.

<!-- semconv metric.http.client.open_connections(metric_table) -->
| Name     | Instrument Type | Unit (UCUM) | Description    |
| -------- | --------------- | ----------- | -------------- |
| `http.client.open_connections` | UpDownCounter | `{connection}` | Number of outbound HTTP connections that are currently active or idle on the client. |
<!-- endsemconv -->

<!-- semconv metric.http.client.open_connections(full) -->
| Attribute  | Type | Description  | Examples  | Requirement Level |
|---|---|---|---|---|
| [`http.connection.state`](../attributes-registry/http.md) | string | State of the HTTP connection in the HTTP connection pool. | `active`; `idle` | Required |
| [`network.peer.address`](../attributes-registry/network.md) | string | Peer address of the network connection - IP address or Unix domain socket name. | `10.1.2.80`; `/tmp/my.sock` | Recommended |
| [`network.protocol.version`](../attributes-registry/network.md) | string | Version of the protocol specified in `network.protocol.name`. [1] | `3.1.1` | Recommended |
| [`server.address`](../attributes-registry/server.md) | string | Server domain name if available without reverse DNS lookup; otherwise, IP address or Unix domain socket name. [2] | `example.com`; `10.1.2.80`; `/tmp/my.sock` | Required |
| [`server.port`](../attributes-registry/server.md) | int | Port identifier of the ["URI origin"](https://www.rfc-editor.org/rfc/rfc9110.html#name-uri-origin) HTTP request is sent to. [3] | `80`; `8080`; `443` | Required |
| [`url.scheme`](../attributes-registry/url.md) | string | The [URI scheme](https://www.rfc-editor.org/rfc/rfc3986#section-3.1) component identifying the used protocol. | `http`; `https` | Opt-In |

**[1]:** `network.protocol.version` refers to the version of the protocol used and might be different from the protocol client's version. If the HTTP client has a version of `0.27.2`, but sends HTTP version `1.1`, this attribute should be set to `1.1`.

**[2]:** When observed from the client side, and when communicating through an intermediary, `server.address` SHOULD represent the server address behind any intermediaries, for example proxies, if it's available.

**[3]:** When observed from the client side, and when communicating through an intermediary, `server.port` SHOULD represent the server port behind any intermediaries, for example proxies, if it's available.

`http.connection.state` has the following list of well-known values. If one of them applies, then the respective value MUST be used, otherwise a custom value MAY be used.

| Value  | Description |
|---|---|
| `active` | active state. |
| `idle` | idle state. |
<!-- endsemconv -->

### Metric: `http.client.connection.duration`

This metric SHOULD be specified with
[`ExplicitBucketBoundaries`](https://github.com/open-telemetry/opentelemetry-specification/tree/v1.26.0/specification/metrics/api.md#instrument-advisory-parameters)
of `[ 0.01, 0.02, 0.05, 0.1, 0.2, 0.5, 1, 2, 5, 10, 30, 60, 120, 300 ]`.

**Status**: [Experimental][DocumentStatus]

This metric is optional.

<!-- semconv metric.http.client.connection.duration(metric_table) -->
| Name     | Instrument Type | Unit (UCUM) | Description    | Stability |
| -------- | --------------- | ----------- | -------------- | --------- |
| `http.client.connection.duration` | Histogram | `s` | The duration of the successfully established outbound HTTP connections. | ![Experimental](https://img.shields.io/badge/-experimental-blue) |
<!-- endsemconv -->

<!-- semconv metric.http.client.connection.duration(full) -->
| Attribute  | Type | Description  | Examples  | [Requirement Level](https://opentelemetry.io/docs/specs/semconv/general/attribute-requirement-level/) | Stability |
|---|---|---|---|---|---|
| [`server.address`](../attributes-registry/server.md) | string | Server domain name if available without reverse DNS lookup; otherwise, IP address or Unix domain socket name. [1] | `example.com`; `10.1.2.80`; `/tmp/my.sock` | `Required` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`server.port`](../attributes-registry/server.md) | int | Port identifier of the ["URI origin"](https://www.rfc-editor.org/rfc/rfc9110.html#name-uri-origin) HTTP request is sent to. [2] | `80`; `8080`; `443` | `Required` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`network.peer.address`](../attributes-registry/network.md) | string | Peer address of the network connection - IP address or Unix domain socket name. | `10.1.2.80`; `/tmp/my.sock` | `Recommended` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`network.protocol.version`](../attributes-registry/network.md) | string | Version of the protocol specified in `network.protocol.name`. [3] | `3.1.1` | `Recommended` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |
| [`url.scheme`](../attributes-registry/url.md) | string | The [URI scheme](https://www.rfc-editor.org/rfc/rfc3986#section-3.1) component identifying the used protocol. | `http`; `https` | `Opt-In` | ![Stable](https://img.shields.io/badge/-stable-lightgreen) |

**[1]:** When observed from the client side, and when communicating through an intermediary, `server.address` SHOULD represent the server address behind any intermediaries, for example proxies, if it's available.

**[2]:** When observed from the client side, and when communicating through an intermediary, `server.port` SHOULD represent the server port behind any intermediaries, for example proxies, if it's available.

**[3]:** `network.protocol.version` refers to the version of the protocol used and might be different from the protocol client's version. If the HTTP client has a version of `0.27.2`, but sends HTTP version `1.1`, this attribute should be set to `1.1`.
<!-- endsemconv -->

### Metric: `http.client.active_requests`

**Status**: [Experimental][DocumentStatus]

This metric is optional.

<!-- semconv metric.http.client.active_requests(metric_table) -->
| Name     | Instrument Type | Unit (UCUM) | Description    |
| -------- | --------------- | ----------- | -------------- |
| `http.client.active_requests` | UpDownCounter | `{request}` | Number of active HTTP requests. |
<!-- endsemconv -->

<!-- semconv metric.http.client.active_requests(full) -->
| Attribute  | Type | Description  | Examples  | Requirement Level |
|---|---|---|---|---|
| [`http.request.method`](../attributes-registry/http.md) | string | HTTP request method. [1] | `GET`; `POST`; `HEAD` | Recommended |
| [`server.address`](../attributes-registry/server.md) | string | Server domain name if available without reverse DNS lookup; otherwise, IP address or Unix domain socket name. [2] | `example.com`; `10.1.2.80`; `/tmp/my.sock` | Required |
| [`server.port`](../attributes-registry/server.md) | int | Port identifier of the ["URI origin"](https://www.rfc-editor.org/rfc/rfc9110.html#name-uri-origin) HTTP request is sent to. [3] | `80`; `8080`; `443` | Required |
| [`url.scheme`](../attributes-registry/url.md) | string | The [URI scheme](https://www.rfc-editor.org/rfc/rfc3986#section-3.1) component identifying the used protocol. | `http`; `https` | Opt-In |

**[1]:** HTTP request method value SHOULD be "known" to the instrumentation.
By default, this convention defines "known" methods as the ones listed in [RFC9110](https://www.rfc-editor.org/rfc/rfc9110.html#name-methods)
and the PATCH method defined in [RFC5789](https://www.rfc-editor.org/rfc/rfc5789.html).

If the HTTP request method is not known to instrumentation, it MUST set the `http.request.method` attribute to `_OTHER`.

If the HTTP instrumentation could end up converting valid HTTP request methods to `_OTHER`, then it MUST provide a way to override
the list of known HTTP methods. If this override is done via environment variable, then the environment variable MUST be named
OTEL_INSTRUMENTATION_HTTP_KNOWN_METHODS and support a comma-separated list of case-sensitive known HTTP methods
(this list MUST be a full override of the default known method, it is not a list of known methods in addition to the defaults).

HTTP method names are case-sensitive and `http.request.method` attribute value MUST match a known HTTP method name exactly.
Instrumentations for specific web frameworks that consider HTTP methods to be case insensitive, SHOULD populate a canonical equivalent.
Tracing instrumentations that do so, MUST also set `http.request.method_original` to the original value.

**[2]:** When observed from the client side, and when communicating through an intermediary, `server.address` SHOULD represent the server address behind any intermediaries, for example proxies, if it's available.

**[3]:** When observed from the client side, and when communicating through an intermediary, `server.port` SHOULD represent the server port behind any intermediaries, for example proxies, if it's available.

`http.request.method` has the following list of well-known values. If one of them applies, then the respective value MUST be used, otherwise a custom value MAY be used.

| Value  | Description |
|---|---|
| `CONNECT` | CONNECT method. |
| `DELETE` | DELETE method. |
| `GET` | GET method. |
| `HEAD` | HEAD method. |
| `OPTIONS` | OPTIONS method. |
| `PATCH` | PATCH method. |
| `POST` | POST method. |
| `PUT` | PUT method. |
| `TRACE` | TRACE method. |
| `_OTHER` | Any HTTP method that the instrumentation has no prior knowledge of. |
<!-- endsemconv -->

[DocumentStatus]: https://github.com/open-telemetry/opentelemetry-specification/tree/v1.26.0/specification/document-status.md
