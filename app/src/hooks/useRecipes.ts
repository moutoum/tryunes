import {useContext} from "react";
import {APIContext} from "../providers/APIProvider";
import {RecipeState} from "../reducers/reducer";

export default function useRecipes(): RecipeState {
    const [state] = useContext(APIContext);
    return state.recipes;
}