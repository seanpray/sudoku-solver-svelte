import App from './App.svelte';

// const init = async () => {
//
//     const app = new App({
//         target: document.body,
//         props: {
//           // https://svelte.dev/docs#Creating_a_component
//           greet: solverwasm
//         }
//     });
// };
//
// init();
 const app = new App({
 	target: document.body,
 	props: {}
 });
export default app;
