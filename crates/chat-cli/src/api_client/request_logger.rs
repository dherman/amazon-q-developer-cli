use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::interceptors::Intercept;
use aws_smithy_runtime_api::client::interceptors::context::{BeforeTransmitInterceptorContextMut, BeforeTransmitInterceptorContextRef};
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_types::config_bag::ConfigBag;
use tracing::info;

/// An interceptor that logs the complete JSON request payload for debugging
#[derive(Debug, Clone, Default)]
pub struct RequestLoggerInterceptor {
    enabled: bool,
}

impl RequestLoggerInterceptor {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl Intercept for RequestLoggerInterceptor {
    fn name(&self) -> &'static str {
        "RequestLoggerInterceptor"
    }

    fn modify_before_signing(
        &self,
        context: &mut BeforeTransmitInterceptorContextMut<'_>,
        _runtime_components: &RuntimeComponents,
        _cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        if !self.enabled {
            return Ok(());
        }

        let request = context.request();
        
        // Log the request URI and method
        info!("=== API Request Debug ===");
        info!("URI: {}", request.uri());
        info!("Method: {}", request.method());
        
        // Log headers (excluding sensitive ones)
        info!("Headers:");
        for (name, value) in request.headers() {
            // Skip sensitive headers
            let name_lower = name.to_string().to_lowercase();
            if name_lower.contains("authorization") 
                || name_lower.contains("x-amz-security-token") {
                info!("  {}: [REDACTED]", name);
            } else {
                info!("  {}: {}", name, value);
            }
        }
        
        // Log the request body
        if let Some(body_bytes) = request.body().bytes() {
            match std::str::from_utf8(body_bytes) {
                Ok(body_str) => {
                    info!("Body (raw):");
                    info!("{}", body_str);
                    
                    // Try to pretty-print if it's JSON
                    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(body_str) {
                        if let Ok(pretty_json) = serde_json::to_string_pretty(&json_value) {
                            info!("Body (pretty-printed JSON):");
                            info!("{}", pretty_json);
                        }
                    }
                }
                Err(_) => {
                    info!("Body: [Binary data, {} bytes]", body_bytes.len());
                }
            }
        } else {
            info!("Body: [Empty]");
        }
        
        info!("=== End API Request Debug ===");
        
        Ok(())
    }
    
    fn read_before_transmit(
        &self,
        context: &BeforeTransmitInterceptorContextRef<'_>,
        _runtime_components: &RuntimeComponents,
        _cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        if !self.enabled {
            return Ok(());
        }

        let request = context.request();
        
        // For now, we'll just log in read_before_transmit since it has the final request
        
        Ok(())
    }
}