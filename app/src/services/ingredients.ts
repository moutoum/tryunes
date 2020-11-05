import {get} from "./common";
import {Ingredient} from "../models/ingredients";

export async function list() {
    return await get<Array<Ingredient>>("/ingredients");
}