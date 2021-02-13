import './footer.css';

const Footer = () => {
    const date = new Date();
    const year = date.getFullYear();

    return (
        <footer>
            <span id="copyright">Copyright ©{year}</span>
        </footer>
    );
};

export default Footer;
export { Footer };