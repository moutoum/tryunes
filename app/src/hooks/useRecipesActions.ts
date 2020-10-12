import {useCallback, useContext} from "react";
import {RecipeCreationForm} from "../models/recipes";
import {create} from "../services/recipes";
import {APIContext} from "../providers/APIProvider";

export default function useRecipesActions() {
    const [, dispatch] = useContext(APIContext);

    const createFn = useCallback(async (recipe: RecipeCreationForm) => {
        dispatch({type: "INSERT_RECIPE", recipe: await create(recipe)})
    }, [dispatch])

    return {createFn};
}