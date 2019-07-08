import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    user: {
      get token() {
        return localStorage.getItem("token");
      },
      get pkKey() {
        return localStorage.getItem("pk_key");
      }
    }
  },
  mutations: {
    setUser(state, {token, pk_key}) {
      localStorage.setItem("token", token);
      localStorage.setItem("pk_key", pk_key);
    }
  },
  actions: {
    userLogin(context, {}) {

    }
  }
})
