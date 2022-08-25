import __init__
import model

if __name__ == "__main__":
    with model.RecwaveModelService() as servicer:
        servicer.serve()
