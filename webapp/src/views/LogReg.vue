<template>
    <div class="login-form">
        <el-tabs v-model="type" @tab-click="tabChange" stretch>
            <el-tab-pane label="登录" name="login" v-loading="loading.login">
                <el-input v-model="form.phone" placeholder="手机号" clearable></el-input>
                <el-input v-model="form.loginPassword" placeholder="密码" show-password></el-input>
                <el-checkbox v-model="form.remember">记住我</el-checkbox>
                <el-button type="primary" @click="doLogin">登录</el-button>
            </el-tab-pane>

            <el-tab-pane label="注册" name="signup" v-loading="loading.signup">
                <el-input v-model="form.phone" placeholder="手机号" clearable @blur="checkPhone"></el-input>
                <span v-show="phoneHasError" class="error-msg">{{this.errorMsg}}</span>
                <el-input v-model="form.signupPassword" placeholder="4 - 18 位密码" show-password></el-input>
                <el-row>
                    <el-col :span="17">
                        <el-input v-model="form.code" placeholder="验证码"></el-input>
                    </el-col>
                    <el-col :span="7" class="btn-get-code" v-loading="loading.getCode">
                        <el-button @click="getCode">获取短信验证码</el-button>
                    </el-col>
                </el-row>
                <div id="vaptchaContainer" v-show="captchaShow">
                    <div class="vaptcha-init-main" v-if="captchaShow">
                        <div class="vaptcha-init-loading">
                            <img src="https://cdn.vaptcha.com/vaptcha-loading.gif"/>
                            <span class="vaptcha-text">验证码加载中...</span>
                        </div>
                    </div>
                </div>
                <el-button type="primary" @click="doSignup">注册</el-button>
            </el-tab-pane>
        </el-tabs>
    </div>
</template>

<script>
    import '../thirdparty/v2'
    import api from '../assets/api'
    import Crypto from '../assets/crypto'

    export default {
        data() {
            return {
                type: (this.$route.query.type === "signup" ? "signup" : "login"),

                loading: {
                    login: false,
                    signup: false,
                    getCode: false
                },

                captcha: null,
                captchaShow: false,

                form: {
                    phone: "",
                    loginPassword: "",
                    signupPassword: "",
                    code: "",
                    remember: true,
                },

                token: '',
                phoneHasError: false,
                errorMsg: "",
            }
        },
        methods: {
            typeChange(route) {
                this.type = (route.query.type === "signup" ? "signup" : "login");
            },

            tabChange() {
                this.$router.push({path: '/user/login', query: {type: this.type}});
            },

            async doLogin() {
                const result = await api({
                    url: "user/login",
                    post: {
                        phone: this.form.phone,
                        password: this.form.loginPassword,
                    }
                });
                if (result.code === "0") {
                    this.$message.success("登陆成功");
                    //TODO: save token and pk_key into local storage
                    //TODO: route to home page
                    sessionStorage.setItem("pk_key", result.data.pk_key);
                    sessionStorage.setItem("token", result.data.token);
                }
                else {
                    this.$message.error(result.msg);
                }
            },

            async doSignup() {
                this.loading.signup = true;
                const result = await api({
                    url: "user/signup",
                    post: {
                        phone: this.form.phone,
                        password: this.form.signupPassword,
                        code: this.form.code,
                    }
                });
                if (result.code === "0") {
                    this.$message.success(result.msg);
                    this.form.signupPassword = "";
                    this.form.code = "";
                    this.type = "login";
                }
                else {
                   this.$message.error(result.msg);
                }
                this.loading.signup = false;
            },

            async checkPhone() {
                const result = await api({
                    url: "user/check_phone",
                    post: {
                        phone: this.form.phone
                    }
                });
                if(result.code !== "0") {
                    this.phoneHasError = true;
                    this.errorMsg = result.msg;
                    return false;
                }
                else {
                    this.phoneHasError = false;
                    this.errorMsg = "";
                    return true;
                }
            },

            async getCode() {
                await this.checkPhone();
                if(this.phoneHasError) {
                    return;
                }
                this.loading.getCode = true;
                this.captchaShow = true;
                const _this = this;
                this.captcha.then(function (vaptchaObj) {
                    vaptchaObj.listen('pass', function () {
                        //get sms code if pass captcha
                        _this.token = vaptchaObj.getToken();
                        _this.validateCaptcha();
                    });
                });
                this.captcha.then(function (obj) {
                   obj.render();
                });
                this.loading.getCode = false;
            },

            async validateCaptcha() {
                const result = await api({
                    url: "user/valid_captcha",
                    post: {
                        token: this.token,
                        phone: this.form.phone,
                    }
                });
                if (result.code === "0") {
                    this.$message.success("验证码发送成功");
                    this.captcha.then(function (obj) {
                        obj.destroy();
                    });
                    this.captchaShow = false;
                }
                else {
                    this.$message.error(result.msg);
                    this.captcha.then(function (obj) {
                        obj.reset();
                    })
                }
            },
        },
        mounted() {
            this.captcha = vaptcha({
                vid: '5ab21bc4a485d4287849b213',
                type: 'embed',
                container: '#vaptchaContainer'
            });
        },
        watch: {
            '$route'(to, from) {
                this.typeChange(to);
            }
        }
    }
</script>


<style lang="scss">
    .login-form {
        max-width: 500px;
        margin: 0 auto;
        .el-tab-pane {
            > * {
                display: block;
                margin-top: 10px;
                width: 100%;
            }
            .el-loading-mask {
                margin-top: 0;
            }
            .btn-get-code {
                padding-left: 8px;
                .el-button {
                    width: 100%;
                }
            }
        }
        .error-msg {
            color: red;
            font-size: small;
            text-align: end;
        }
    }

    #vaptchaContainer {
        width: 500px;
    }
</style>
