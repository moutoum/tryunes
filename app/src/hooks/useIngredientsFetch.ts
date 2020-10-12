import {useContext, useEffect} from "react";
import {APIContext} from "../providers/APIProvider";
import {list} from "../services/ingredients";

export default function useRecipesFetch(): void {
    const [, dispatch] = useContext(APIContext);

    useEffect(() => {
        (async () => {
            try {
                dispatch({type: "FETCH_INGREDIENTS_REQUEST"});
                const {data} = await list();
                dispatch({type: "FETCH_INGREDIENTS_SUCCESS", ingredients: data});
            } catch (e) {
                dispatch({type: "FETCH_INGREDIENTS_FAILURE", error: e.message});
            }
        })()
    }, [dispatch])
}