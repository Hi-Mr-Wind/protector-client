use base64::Engine;
use base64::engine::general_purpose;
use rsa::{PaddingScheme, RsaPrivateKey};
use rsa::pkcs8::{FromPrivateKey};

/// 使用私钥解密字符串内容
fn decrypt_string(private_key_pem: &str, encrypted_base64: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 从PEM格式加载私钥
    let private_key = RsaPrivateKey::from_pkcs8_pem(private_key_pem)?;

    // 解码Base64编码的加密字符串
    let encrypted_data = general_purpose::STANDARD.decode(encrypted_base64)?;

    // 使用私钥解密数据
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let decrypted_data = private_key.decrypt(padding, &encrypted_data)?;

    // 将解密后的数据转换为字符串
    let decrypted_string = str::from_utf8(&decrypted_data)?.to_string();

    Ok(decrypted_string)
}