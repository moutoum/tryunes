import axios from 'axios';

const instance = axios.create();
instance.defaults.responseType = 'json';
instance.defaults.validateStatus = (status) => status >= 200 && status <= 300;
instance.defaults.baseURL = 'http://localhost:8000/api';

export async function get(url: string) {
    return await instance.get(url);
}
