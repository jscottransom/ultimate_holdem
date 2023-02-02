from PyQt5.QtWidgets import *
from PyQt5.QtGui import QPixmap
import sys
import ultimate_holdem


def build_deck() -> list:
    """
    Builds a standard deck of cards used for gameplay.

    Args:
        None

    Returns:
        list: A list of Card objects represeenting playable values
    """
    number_cards = ultimate_holdem.build_number_cards()
    face_cards = ultimate_holdem.build_face_cards()

    return [number_cards + face_cards]

 
 
class Window(QMainWindow):
    def __init__(self):
        super().__init__()
 
        self.acceptDrops()
        # set the title
        self.setWindowTitle("Image")
 
        # setting  the geometry of window
        self.setGeometry(0, 0, 100, 200)
 
        # creating label
        self.label = QLabel(self)
         
        
        self.img = self.get_image_path_to_render(self.build_deck())
        
        # loading image
        self.pixmap = QPixmap(self.img)
 
        # adding image to label
        self.label.setPixmap(self.pixmap)
 
        # Optional, resize label to image size
        self.label.resize(self.pixmap.width(),
                          self.pixmap.height())

        self.pixmap.scaledToWidth(64)
 
        # show all the widgets
        self.show()

    def build_deck(self) -> list:
        """
        Builds a standard deck of cards used for gameplay.

        Args:
            None

        Returns:
            list: A list of Card objects represeenting playable values
        """
        number_cards = ultimate_holdem.build_number_cards()
        face_cards = ultimate_holdem.build_face_cards()

        return [number_cards + face_cards]

    def get_image_path_to_render(self, deck) -> str:
        
        return deck[0][0].get_image_path()

if __name__ == '__main__':
    # create pyqt5 app
    App = QApplication(sys.argv)
 
    # create the instance of our Window
    window = Window()
 
    # start the app
    sys.exit(App.exec())
    
   

