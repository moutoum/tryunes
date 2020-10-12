import React, {useCallback} from "react";
import {Button, Col, Form} from "react-bootstrap";
import {FieldArrayRenderProps} from "formik";
import useIngredients from "../hooks/useIngredients";
import useIngredientsFetch from "../hooks/useIngredientsFetch";
import {Trash} from "react-bootstrap-icons";
import {RecipeIngredient} from "../models/recipes";

const initialRecipeIngredient: RecipeIngredient = {
    ingredient_id: 0,
    quantity: "",
};

const RecipeIngredientsForm: React.FC<FieldArrayRenderProps> = (props) => {
    const {
        push,
        remove,
        form: {values, handleChange, handleBlur},
        name,
    } = props;

    const ingredientState = useIngredients();
    const removeIngredient = useCallback((index) => () => remove(index), [remove]);
    const insertIngredient = useCallback(() => push(initialRecipeIngredient), [push]);

    useIngredientsFetch();

    if (ingredientState.status !== "success") {
        return null;
    }

    return (<>
            {values.ingredients.map((ingredient: RecipeIngredient, i: number) => (
                <Form.Row key={i}>
                    <Form.Group as={Col}>
                        <Form.Control
                            as="select"
                            custom
                            name={`${name}.${i}.ingredient_id`}
                            value={ingredient.ingredient_id}
                            onChange={handleChange}
                            onBlur={handleBlur}
                        >
                            {ingredientState.data.map(ingredient =>
                                <option
                                    value={ingredient.id}
                                    key={ingredient.id}
                                >{ingredient.name}</option>)}
                        </Form.Control>
                    </Form.Group>

                    <Form.Group as={Col}>
                        <Form.Control
                            type="text"
                            name={`${name}.${i}.quantity`}
                            value={ingredient.quantity}
                            onChange={handleChange}
                            onBlur={handleBlur}
                            placeholder={"500g..."}
                        />
                    </Form.Group>

                    <Col xs="auto">
                        <Button variant="outline-danger" onClick={removeIngredient(i)}><Trash/></Button>
                    </Col>
                </Form.Row>
            ))}
            <Button variant="outline-secondary" onClick={insertIngredient}>Ajouter</Button>
        </>
    )
}

export default RecipeIngredientsForm;