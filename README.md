# Ejercicio Final Substrate - Crowdfund Pallet

Este ejercicio tiene como objetivo desarrollar y poner a prueba tus habilidades para trabajar con pallets en Substrate, utilizando como ejemplo la creación de algunas funciones de una aplicación de financiamiento colectivo (crowdfunding). El crowdfunding es una metodología que permite recolectar fondos para proyectos o causas a través de contribuciones individuales.

Vas a trabajar en el archivo `src/lib.rs` del crowdfund pallet, proporcionado en el repositorio. Tu misión es asegurarte de que los tests que se encuentran en el pallet se ejecuten correctamente.

## Modo de Trabajo

Vas a tener que clonar este repositorio y trabajar en tu propia copia privada. El dia de la entrega, hacé tu repositorio público para permitir la evaluación.

## Desarrollo del Pallet

Los tests que se proveen son `crear_proyecto_funciona` y `apoyar_proyecto_funciona`. Estos validan las funciones esenciales para la creación y financiamiento de proyectos.

### Crear Proyecto

La función `crear_proyecto` toma un parámetro: un nombre para el proyecto. Este nombre debe tener una longitud mínima y máxima, lo que se valida en los tests proporcionados. Vas a tener que manejar los casos de error para nombres demasiado cortos y largos, retornando `Error::<Test>::NombreMuyCorto` y `Error::<Test>::NombreMuyLargo` respectivamente. Si el nombre es válido, vas a tener que crear un nuevo proyecto con ese nombre y almacenarlo en el estado del pallet. El estado del proyecto inicialmente será cero, indicando que no ha recibido financiamiento.

### Apoyar Proyecto

La función `apoyar_proyecto` recibe como parámetros el nombre del proyecto que se quiere apoyar y la cantidad de fondos que se quieren aportar. Esta función deberá buscar el proyecto por su nombre y, si existe, incrementar su financiamiento en la cantidad aportada. En caso de que el proyecto no exista, deberá retornar un error.

Además, se provee un archivo `tipos.rs` que define varios tipos que pueden ser útiles para la implementación del pallet. Estos tipos pueden ser utilizados (o no) según sea necesario al implementar las funciones del pallet.

## Pruebas Finales

Finalmente, vas a tener que ejecutar los tests nuevamente en el entorno completo para confirmar que todas las funciones funcionan correctamente.

Recordá, el objetivo de este ejercicio es aprender. Si tenés alguna duda, no dudes en consultar en el canal de Discord. No te quedes atrapado en un problema, siempre estamos acá para ayudar.

Para mas documentacion acerca de como trabajar con este template, es posible chequear el [Substrate node template](https://github.com/substrate-developer-hub/substrate-node-template)

