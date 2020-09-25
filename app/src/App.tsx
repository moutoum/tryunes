import React, {useCallback, useEffect, useState} from 'react';
import {list} from "./services/recipes";
import Recipes from "./components/Recipes";
import RecipePanel from "./components/RecipePanel";

function App() {
    const [recipes, setRecipes] = useState<Array<Recipe>>([]);
    const [showed, setShow] = useState(false);
    const [selected, setSelected] = useState();
    const handleRecipeClick = useCallback((recipeId: number) => {
        if (selected === recipeId) {
            setSelected(undefined);
            setShow(false);
        } else {
            setSelected(recipeId);
            setShow(true);
        }
    }, [setShow, selected]);

    useEffect(() => {
        list().then((response) => setRecipes(response.data))
    }, [])

    return (
        <>
            <Recipes recipes={recipes} onRecipeClicked={handleRecipeClick} style={{ width: '70%'}}/>
            {showed &&
            <div style={{width: '30%', position: 'fixed', backgroundColor: "white", height: '100%', right: 0, top: 0}}>
                {recipes.filter(({id}) => id === selected).map(recipe => <RecipePanel recipe={recipe} />)}
            </div>}
        </>
    );
}

export default App;
