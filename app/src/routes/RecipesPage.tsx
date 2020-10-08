import React from "react";
import CardColumns from "react-bootstrap/CardColumns"
import useRecipes from "../hooks/useRecipes";
import useRecipesFetch from "../hooks/useRecipesFetch";
import RecipeCard from "../components/RecipeCard";

const RecipesPage: React.FC = () => {
    const state = useRecipes();
    useRecipesFetch();

    switch (state.status) {
        case "empty":
            return <>Empty</>;

        case "error":
            return <p>state.error</p>;

        case "loading":
            return <p>Loading...</p>;

        case "success":
            return (
                <CardColumns>
                    {state.data.map((recipe) => <RecipeCard key={recipe.id} recipe={recipe}/>)}
                </CardColumns>
            );
    }
}

export default RecipesPage;