import {get} from "./common";

export async function list() {
    return await get("/ingredients");
}