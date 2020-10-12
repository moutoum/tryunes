import React, {useCallback} from 'react';
import {Button, Card, Col, Form, InputGroup} from "react-bootstrap";
import {FieldArray, FieldArrayRenderProps, Formik} from "formik";
import RecipeIngredientsForm from "./RecipeIngredientsForm";
import useRecipesActions from "../hooks/useRecipesActions";
import {RecipeCreationForm as RecipeCreationFormType} from "../models/recipes";

const initialValues: RecipeCreationFormType = {
    name: "",
    description: "",
    image: "",
    price: 0,
    preparation_duration: 0,
    cooking_duration: 0,
    ingredients: [],
    steps: [],
};

type Props = {
    onRecipeCreated: () => void
}

const RecipeCreationForm: React.FC<Props> = ({ onRecipeCreated }) => {
    const {createFn} = useRecipesActions();
    const submit = useCallback((values) => {
        createFn(values).then(onRecipeCreated);
    }, [createFn, onRecipeCreated]);

    return (
        <Formik
            initialValues={initialValues}
            onSubmit={submit}>
            {({
                  handleSubmit,
                  handleChange,
                  handleBlur,
                  values,
                  touched,
                  errors
              }) => (
                <Form noValidate>
                    <Form.Group>
                        <Form.Label>Nom</Form.Label>
                        <Form.Control
                            type="text"
                            name="name"
                            value={values.name}
                            onChange={handleChange}
                            onBlur={handleBlur}
                            isValid={touched.name && !errors.name}
                        />
                    </Form.Group>

                    <Form.Group>
                        <Form.Label>Description</Form.Label>
                        <Form.Control
                            as="textarea"
                            type="text"
                            rows={2}
                            name="description"
                            value={values.description}
                            onChange={handleChange}
                            onBlur={handleBlur}
                            isValid={touched.description && !errors.description}
                        />
                    </Form.Group>

                    <Form.Group>
                        <Form.Label>Image</Form.Label>
                        <Form.Control
                            type="text"
                            name="image"
                            value={values.image}
                            onChange={handleChange}
                            onBlur={handleBlur}
                            isValid={touched.image && !errors.image}
                        />
                    </Form.Group>

                    <Form.Group>
                        <Form.Label>Prix</Form.Label>
                        <InputGroup>
                            <Form.Control
                                type="number"
                                step={0.25}
                                name="price"
                                value={values.price}
                                onChange={handleChange}
                                onBlur={handleBlur}
                                isValid={touched.price && !errors.price}
                            />
                            <InputGroup.Append>
                                <InputGroup.Text>€</InputGroup.Text>
                            </InputGroup.Append>
                        </InputGroup>
                    </Form.Group>

                    <Form.Row>
                        <Form.Group as={Col}>
                            <Form.Label>Temps de préparation</Form.Label>
                            <InputGroup>
                                <Form.Control
                                    type="number"
                                    name="preparation_duration"
                                    value={values.preparation_duration}
                                    onChange={handleChange}
                                    onBlur={handleBlur}
                                    isValid={touched.preparation_duration && !errors.preparation_duration}
                                />
                                <InputGroup.Append>
                                    <InputGroup.Text>Minutes</InputGroup.Text>
                                </InputGroup.Append>
                            </InputGroup>
                        </Form.Group>

                        <Form.Group as={Col}>
                            <Form.Label>Temps de cuisson</Form.Label>
                            <InputGroup>
                                <Form.Control
                                    type="number"
                                    name="cooking_duration"
                                    value={values.cooking_duration}
                                    onChange={handleChange}
                                    onBlur={handleBlur}
                                    isValid={touched.cooking_duration && !errors.cooking_duration}
                                />
                                <InputGroup.Append>
                                    <InputGroup.Text>Minutes</InputGroup.Text>
                                </InputGroup.Append>
                            </InputGroup>
                        </Form.Group>
                    </Form.Row>

                    <Card>
                        <Card.Header><Card.Title>Ingredients</Card.Title></Card.Header>
                        <Card.Body>
                            <FieldArray name="ingredients" component={RecipeIngredientsForm as React.FC<FieldArrayRenderProps | void>} />
                        </Card.Body>
                    </Card>

                    <Button
                        className="d-block ml-auto mt-2"
                        variant="success"
                        onClick={() => handleSubmit()}
                    >Créer</Button>
                </Form>
            )}
        </Formik>
    );
}

export default RecipeCreationForm;