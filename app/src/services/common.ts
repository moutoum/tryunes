import axios, {AxiosResponse} from 'axios';

const instance = axios.create();
instance.defaults.responseType = 'json';
instance.defaults.validateStatus = (status) => status >= 200 && status <= 300;
instance.defaults.baseURL = 'http://localhost:8000/api';

export async function get<R>(url: string) {
    return await instance.get<any, AxiosResponse<R>>(url);
}

export async function post<T, R>(url: string, data: T): Promise<AxiosResponse<R>> {
    return await instance.post<T, AxiosResponse<R>>(url, data);
}