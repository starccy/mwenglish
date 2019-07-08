const CryptoJS = require('crypto-js');
import {JSEncrypt} from 'jsencrypt';

export default class Crypto {
    constructor() {
        this.aes_key = this.getRandomNumber();
        this.iv_key = "1234567890123456";
    }

    getRandomNumber() {
        let result = "";
        for (let i = 0; i < 16; ++i) {
            result += Math.floor(Math.random() * 16).toString(16);
        }
        return result;
    }

    AESEncryptData(data) {
        let aes_key = CryptoJS.enc.Utf8.parse(this.aes_key);
        let iv_key = CryptoJS.enc.Utf8.parse(this.iv_key);
        let src = CryptoJS.enc.Utf8.parse(data);
        console.log(aes_key, iv_key, src);
        let encrypted = CryptoJS.AES.encrypt(src, aes_key, {iv: iv_key, mode: CryptoJS.mode.CBC, padding: CryptoJS.pad.Pkcs7});
        return encrypted.toString();
    }

    getSKey() {
        let encoded_aes_key = window.btoa(this.aes_key);
        let public_key = localStorage.getItem("pk_key");
        let encryptor = new JSEncrypt();
        encryptor.setPublicKey(window.atob(public_key));
        return encryptor.encrypt(encoded_aes_key);
    }

}