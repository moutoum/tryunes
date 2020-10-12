import {useContext} from "react";
import {APIContext} from "../providers/APIProvider";
import {IngredientState} from "../reducers/reducer";

export default function useIngredients(): IngredientState {
    const [state] = useContext(APIContext);
    return state.ingredients;
}