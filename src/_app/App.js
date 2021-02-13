import './App.css';
import mario from '../misc/mario';
import Header from '../header/header';
import Footer from '../footer/footer';
import Home from '../pages/home/home';

const App = () => {
  mario();

  return (
    <>
      <Header />
      <Home />
      <Footer />
    </>
  );
};

export default App;
export { App };