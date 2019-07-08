import axios from 'axios'

export default async function (params) {
	// const url = (process.server ? 'http://localhost:8000' : '') + `/api/${params.url}`;
	const url = 'http://localhost:8000' + `/api/${params.url}`;
	const axiosData = {
		url: url,
		responseType: 'text'
	};
	axiosData.headers = {};
	if (process.server) {
		axiosData.headers['User-Agent'] = 'Mozilla/5.0 Nuxt Server';
	}
	if (params.cookie) {
		axiosData.headers['Cookie'] = params.cookie;
	}
	if (params.query) {
		axiosData.params = params.query;
	}
	if (params.post) {
		axiosData.data = params.post;
		axiosData.method = 'post';
	} else {
		axiosData.method = 'get';
	}
	const result = await axios.request(axiosData);
	return result.data;
}
