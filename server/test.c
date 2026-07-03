#include <stdio.h>
#include <stdlib.h>

void call_rust(const char *command) {
    char buffer[512];

    FILE *pipe = popen(command, "r");

    if (!pipe) {
        perror("Не удалось запустить Rust-процесс");
        return;
    }

    while (fgets(buffer, sizeof(buffer), pipe) != NULL) {
        printf("Ответ Rust: %s", buffer);
    }

    pclose(pipe);
}

int main() {
    //генерация rsa ключа
    call_rust("../backend/target/release/backend generate_rsa_keys");

    // генерация rsa ключа
    call_rust("../backend/target/release/backend generate_aes_key");
     
    //шифрование aes ключа
    call_rust("../backend/target/release/backend rsa_encrypt_aes_key RSA_PUBLIC_KEY_STUB AES_KEY");

    //расшифровка aes ключа
    call_rust("../backend/target/release/backend rsa_decrypt_aes_key RSA_PRIVATE_KEY_STUB AES_KEY");
    
    //шифрование текста
    call_rust("../backend/target/release/backend aes_encrypt_text AES_KEY \"Какой-то текст\"");

    //расшифровка текста
    call_rust("../backend/target/release/backend aes_decrypt_text AES_KEY \"Какой-то текст\"");

    return 0;
}