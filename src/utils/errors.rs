use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuroraError {
    #[error("Paket bulunamadı: {0}")]
    PackageNotFound(String),

    #[error("İşlem için yönetici (root/polkit) yetkisi gerekiyor.")]
    PermissionDenied,

    #[error("Ağ bağlantı hatası veya depo erişilemiyor: {0}")]
    NetworkError(String),

    #[error("AUR işlemi başarısız oldu: {0}")]
    AurError(String),

    #[error("İşlem kullanıcı tarafından iptal edildi.")]
    UserCancelled,

    #[error("Bilinmeyen sistem hatası: {0}")]
    SystemError(String),
}
