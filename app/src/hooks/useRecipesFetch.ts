import {useContext, useEffect} from "react";
import {APIContext} from "../providers/APIProvider";
import {list} from "../services/recipes";

export default function useRecipesFetch(): void {
    const [, dispatch] = useContext(APIContext);

    useEffect(() => {
        (async () => {
            try {
                dispatch({type: "FETCH_RECIPES_REQUEST"});
                const {data} = await list();
                dispatch({type: "FETCH_RECIPES_SUCCESS", recipes: data});
            } catch (e) {
                dispatch({type: "FETCH_RECIPES_FAILURE", error: e.message});
            }
        })()
    }, [dispatch])
}