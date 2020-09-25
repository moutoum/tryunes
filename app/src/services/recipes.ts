import {get} from "./common";

export async function list() {
    return await get('/recipes');
}

export async function getById(id: number) {
    return await get(`/recipes/${id}`);
}