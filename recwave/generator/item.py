import uuid
import numpy as np
from preference import ItemProperties
from pprint import pprint


def new_item():
    id = str(np.random.randint(1, 1000_000_000))

    # activeness marks how frequent the user interact with merchandises
    popularity = np.exp(np.random.lognormal(mean=1))
    distrib = dict(itemid=id, popularity=popularity)

    for tag, gen in ItemProperties.generators.items():
        distrib[tag] = float(gen())

    return distrib


if __name__ == "__main__":
    pprint(new_item())
