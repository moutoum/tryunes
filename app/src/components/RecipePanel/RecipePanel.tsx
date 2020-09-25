import React from 'react';
import classes from './RecipePanel.module.css';

interface Props {
    recipe: Recipe
}

const RecipePanel: React.FC<Props> = ({recipe}) => (
    <div className={classes.container}>
        {/* Header */}
        <div>
            <div className={classes.content}>
                <h2 className={classes.title}>{recipe.name}</h2>
                <p className={classes.description}>{recipe.description}</p>
            </div>
            <div className={classes.separator}/>
        </div>

        {/* Content */}
        <div className={[classes.ingredients, classes.content].join(" ")} style={{marginTop: '2em'}}>
            {recipe.ingredients.map(ingredient => (
                <div className={classes.ingredient}>
                    <img src={ingredient.image} alt={ingredient.name}/>
                    <p className={classes.label}>{ingredient.name}</p>
                    <p className={classes.quantity}>{ingredient.quantity}</p>
                </div>
            ))}
        </div>
        <div className={classes.content} style={{margin: '1em 0', overflowY: 'auto'}}>
            <ol className={classes.list}>
                {recipe.steps.map(step => <li>{step}</li>)}
            </ol>
        </div>

        {/* Footer */}
        <div className={classes.picture}>
            <img src={recipe.image} alt="caoo"/>
        </div>
    </div>
);

export default RecipePanel;