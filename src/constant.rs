pub const                      VERSION: &str = env!("CARGO_PKG_VERSION");
pub const                 SERVICE_NAME: &str = "ironshield-api";
pub const              HEALTH_ENDPOINT: &str = "/health";
pub const             REQUEST_ENDPOINT: &str = "/request";
pub const            RESPONSE_ENDPOINT: &str = "/response";
pub const         CHALLENGE_DIFFICULTY:  u64 = 5_000_000;

pub const       FAVICON_CACHE_DURATION:  u32 = 86400;   // 1 day.
pub const       OPENAPI_CACHE_DURATION:  u32 = 300;     // 5 minutes.
pub const    SWAGGER_UI_CACHE_DURATION:  u32 = 300;     // 5 minutes.
pub const              SWAGGER_UI_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="description" content="IronShield RESTAPI Swagger UI" />
    <title>REST API | IronShield</title>
    <link rel="icon" type="image/x-icon" href="/favicon.ico">
    <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui.css" />
    <style>
        .swagger-ui .info .title {
            color: #0074E9;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        }
        .swagger-ui .info .description {
            color: #333;
        }
        .swagger-ui .scheme-container {
            background: linear-gradient(90deg, #0074E9 0%, #9ECDFC 100%);
            border: none;
            box-shadow: 0 2px 4px rgba(0,116,233,0.1);
        }
        .swagger-ui .topbar {
            background: linear-gradient(90deg, #0074E9 0%, #9ECDFC 100%);
            border-bottom: 1px solid #e3e3e3;
        }
        .swagger-ui .topbar .download-url-wrapper .download-url-button {
            background: #0074E9;
            border-color: #0074E9;
        }
        .swagger-ui .btn.authorize {
            background: #0074E9;
            border-color: #0074E9;
        }
        .swagger-ui .btn.execute {
            background: #0074E9;
            border-color: #0074E9;
        }
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-bundle.js" crossorigin></script>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-standalone-preset.js" crossorigin></script>
    <script>
        window.onload = () => {
            window.ui = SwaggerUIBundle({
                url: '/api-docs/openapi.json',
                dom_id: '#swagger-ui',
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIStandalonePreset
                ],
                layout: "StandaloneLayout",
                deepLinking: true,
                showExtensions: true,
                showCommonExtensions: true,
                tryItOutEnabled: true,
                filter: true,
                requestInterceptor: (request) => {
                    return request;
                },
                responseInterceptor: (response) => {
                    return response;
                }
            });
        };
    </script>
</body>
</html>"#;